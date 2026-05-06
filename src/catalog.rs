//! Read-only metadata catalog seam.
//!
//! A "file" is the unit of pruning — equivalent to one Iceberg data
//! file or any other column-store object the planner can skip without
//! materialising. Implementations expose per-file statistics (the
//! caller's sketch type `S`, e.g. an HLL + variance + grammar
//! histogram) so the planner can prune *before* paying for I/O.
//!
//! The trait is intentionally generic over `S`. rig-ballista does not
//! prescribe a sketch shape; downstream agents pick whatever
//! statistics their pruner needs. The companion
//! [`InMemoryMetadataCatalog`] is a thread-safe reference
//! implementation suitable for tests and offline harnesses; the
//! Iceberg+Ballista-backed implementation will plug into the same
//! trait once the upstream toolchain stabilises.
//!
//! ```no_run
//! use rig_ballista::catalog::{
//!     FileId, FileStats, InMemoryMetadataCatalog, MetadataCatalog,
//! };
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let cat = InMemoryMetadataCatalog::<u64>::new();
//! cat.insert(FileStats { id: FileId::new(), partition: "auth".into(), sketch: 7 });
//! let files = cat.list_files(Some("auth")).await?;
//! assert_eq!(files.len(), 1);
//! # Ok(()) }
//! ```

use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Stable identifier for a stored file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub Uuid);

impl FileId {
    /// Mint a fresh, random [`FileId`].
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for FileId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for FileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// File-level sketch summary — the Puffin-blob equivalent.
///
/// Generic over the caller's sketch type `S`. The sketch is the
/// *aggregate* over the records within the file; the catalog backend
/// (in-memory today, Iceberg+Puffin tomorrow) is responsible for that
/// aggregation at write time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats<S> {
    /// Stable file identifier.
    pub id: FileId,
    /// Partition tag. Convention is caller-defined; the catalog only
    /// uses string equality for filtering.
    pub partition: String,
    /// Caller-supplied summary statistics (HLL, variance, histograms,
    /// etc.).
    pub sketch: S,
}

/// Errors surfaced by [`MetadataCatalog`] implementations.
#[derive(Debug, Error)]
pub enum StorageError {
    /// The catalog backend failed to enumerate or read metadata.
    ///
    /// The wrapped source is preserved so `Error::source()` walks to the
    /// originating I/O / object-store / Iceberg failure.
    #[error("catalog backend error: {0}")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),

    /// Direct lookup against an unknown file id.
    #[error("file not found: {0}")]
    NotFound(FileId),
}

impl StorageError {
    /// Wrap an arbitrary backend error as [`StorageError::Backend`].
    pub fn backend<E>(err: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::Backend(err.into())
    }
}

/// Read-only metadata catalog. Implementations expose Puffin/Iceberg
/// statistics so the planner can prune before materialising blocks.
///
/// `S` is the caller's sketch type. The trait is `async` because real
/// Iceberg-backed implementations will perform network I/O against an
/// object store.
#[async_trait]
pub trait MetadataCatalog<S>: Send + Sync
where
    S: Send + Sync + Clone + 'static,
{
    /// List every file matching `partition`, or all files when `None`.
    async fn list_files(&self, partition: Option<&str>) -> Result<Vec<FileStats<S>>, StorageError>;

    /// Fetch stats for one file by id.
    async fn get(&self, id: FileId) -> Result<FileStats<S>, StorageError>;
}

/// In-memory [`MetadataCatalog`] used by tests, examples, and offline
/// harnesses.
///
/// Backed by a [`DashMap`] keyed on [`FileId`] so concurrent inserts
/// from multiple writer tasks are safe without coarse locking.
#[derive(Debug)]
pub struct InMemoryMetadataCatalog<S> {
    files: DashMap<FileId, FileStats<S>>,
}

impl<S> Default for InMemoryMetadataCatalog<S> {
    fn default() -> Self {
        Self {
            files: DashMap::new(),
        }
    }
}

impl<S> InMemoryMetadataCatalog<S>
where
    S: Send + Sync + Clone + 'static,
{
    /// Create a fresh catalog wrapped in [`Arc`] for cheap cloning.
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Insert (or overwrite) the stats for one file.
    pub fn insert(&self, stats: FileStats<S>) {
        self.files.insert(stats.id, stats);
    }

    /// Number of files currently catalogued.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Whether the catalog is empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}

#[async_trait]
impl<S> MetadataCatalog<S> for InMemoryMetadataCatalog<S>
where
    S: Send + Sync + Clone + 'static,
{
    async fn list_files(&self, partition: Option<&str>) -> Result<Vec<FileStats<S>>, StorageError> {
        let files = match partition {
            None => self.files.iter().map(|kv| kv.value().clone()).collect(),
            Some(p) => self
                .files
                .iter()
                .filter(|kv| kv.value().partition == p)
                .map(|kv| kv.value().clone())
                .collect(),
        };
        Ok(files)
    }

    async fn get(&self, id: FileId) -> Result<FileStats<S>, StorageError> {
        self.files
            .get(&id)
            .map(|kv| kv.value().clone())
            .ok_or(StorageError::NotFound(id))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn stats(partition: &str, sketch: u32) -> FileStats<u32> {
        FileStats {
            id: FileId::new(),
            partition: partition.into(),
            sketch,
        }
    }

    #[tokio::test]
    async fn list_filters_by_partition() {
        let cat = InMemoryMetadataCatalog::<u32>::new();
        cat.insert(stats("a", 1));
        cat.insert(stats("a", 2));
        cat.insert(stats("b", 3));
        assert_eq!(cat.list_files(None).await.unwrap().len(), 3);
        assert_eq!(cat.list_files(Some("a")).await.unwrap().len(), 2);
        assert_eq!(cat.list_files(Some("b")).await.unwrap().len(), 1);
        assert_eq!(cat.list_files(Some("missing")).await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn get_returns_not_found_for_unknown() {
        let cat = InMemoryMetadataCatalog::<u32>::new();
        match cat.get(FileId::new()).await {
            Err(StorageError::NotFound(_)) => {}
            other => panic!("expected NotFound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn insert_and_get_round_trips() {
        let cat = InMemoryMetadataCatalog::<u32>::new();
        let s = stats("p", 42);
        let id = s.id;
        cat.insert(s);
        let got = cat.get(id).await.unwrap();
        assert_eq!(got.sketch, 42);
        assert_eq!(got.partition, "p");
    }

    #[tokio::test]
    async fn len_and_is_empty_track_inserts() {
        let cat = InMemoryMetadataCatalog::<u32>::new();
        assert!(cat.is_empty());
        cat.insert(stats("p", 1));
        assert_eq!(cat.len(), 1);
        assert!(!cat.is_empty());
    }
}
