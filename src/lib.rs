//! # rig-ballista
//!
//! Apache Ballista + DataFusion + Iceberg companion crate for
//! [`rig-compose`](https://crates.io/crates/rig-compose).
//!
//! **Status:** the [`catalog::MetadataCatalog`] trait and the
//! [`catalog::InMemoryMetadataCatalog`] reference implementation ship
//! today. The Iceberg + Ballista-backed catalog will plug into the
//! same trait once the upstream toolchain stabilises.
//!
//! ## Why a separate crate
//! - **MSRV isolation.** `iceberg-rust` 0.9 currently requires rustc 1.92;
//!   pinning that into the rig-compose tree would force every consumer
//!   onto the same toolchain.
//! - **Compile-cost isolation.** Ballista pulls in DataFusion, Arrow,
//!   gRPC, Parquet — none of which the kernel needs.
//! - **Boundary discipline.** Ballista is a query-engine boundary, not a
//!   file-format boundary. Keeping it behind a trait keeps the skill/tool
//!   surface clean.
//!
//! ## Pruning seam
//!
//! Downstream agents store per-file sketches (HLL, variance, grammar
//! histograms — whatever their pruner needs) and reach the catalog
//! through [`catalog::MetadataCatalog`]:
//!
//! ```no_run
//! use rig_ballista::{FileId, FileStats, InMemoryMetadataCatalog, MetadataCatalog};
//!
//! #[derive(Clone)]
//! struct MySketch { distinct: u64, variance: f64 }
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let cat = InMemoryMetadataCatalog::<MySketch>::new();
//! cat.insert(FileStats {
//!     id: FileId::new(),
//!     partition: "auth-edge".into(),
//!     sketch: MySketch { distinct: 1024, variance: 0.7 },
//! });
//! let files = cat.list_files(None).await?;
//! let _ = files;
//! # Ok(()) }
//! ```

#![doc(html_root_url = "https://docs.rs/rig-ballista/0.1.0")]
#![deny(missing_docs)]

pub mod catalog;

pub use catalog::{FileId, FileStats, InMemoryMetadataCatalog, MetadataCatalog, StorageError};

/// Placeholder type retained for backward compatibility with the
/// initial `0.1.0` scaffolding release. Prefer
/// [`catalog::InMemoryMetadataCatalog`] for new code; this type will
/// be removed in `0.2`.
#[deprecated(
    since = "0.1.1",
    note = "use `rig_ballista::catalog::InMemoryMetadataCatalog` instead"
)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PlaceholderCatalog;

#[allow(deprecated)]
impl PlaceholderCatalog {
    /// Mint a new placeholder. Prefer
    /// [`catalog::InMemoryMetadataCatalog::new`] for new code.
    pub const fn new() -> Self {
        Self
    }
}
