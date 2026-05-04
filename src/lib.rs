//! # rig-ballista
//!
//! Apache Ballista + DataFusion + Iceberg companion crate for
//! [`rig-compose`](https://crates.io/crates/rig-compose).
//!
//! **Status:** scaffolding. The intent is to provide a `MetadataCatalog`-
//! shaped trait impl that reads Iceberg tables (via `datafusion-iceberg`)
//! and pushes prunable scans through a Ballista distributed query engine.
//! The seam exists today as `azreal::storage::MetadataCatalog`; this
//! crate will plug behind it without re-exporting Iceberg/Ballista types
//! into the rig-compose surface.
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
//! ## Planned surface
//!
//! ```ignore
//! pub trait MetadataCatalog: Send + Sync {
//!     async fn list_files(&self, partition: Option<&str>) -> Result<Vec<FileStats>, StorageError>;
//!     async fn get(&self, id: FileId) -> Result<FileStats, StorageError>;
//! }
//!
//! pub struct BallistaIcebergCatalog { /* ... */ }
//! impl MetadataCatalog for BallistaIcebergCatalog { /* ... */ }
//! ```
//!
//! Concrete code will land once a throwaway companion verifies the
//! `iceberg-rust` + `datafusion-iceberg` + `ballista` combination
//! compiles on a recent stable toolchain.

#![doc(html_root_url = "https://docs.rs/rig-ballista/0.1.0")]

/// Placeholder until the real catalog implementation lands.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlaceholderCatalog;

impl PlaceholderCatalog {
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_constructs() {
        let _ = PlaceholderCatalog::new();
    }
}
