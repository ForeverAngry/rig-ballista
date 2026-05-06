# Changelog

All notable changes to `rig-ballista` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Versions are managed automatically by [release-plz](https://release-plz.dev/)
from [Conventional Commits](https://www.conventionalcommits.org/).

## [Unreleased]

### Added

- `catalog` module: domain-neutral `MetadataCatalog<S>` trait,
  `FileId`/`FileStats<S>`/`StorageError` types, and an
  `InMemoryMetadataCatalog<S>` reference implementation. Lifted from
  Azrael so any rig-compose agent can prune column-store scans against
  per-file sketches without coupling to Iceberg or Ballista. The
  Iceberg + Ballista-backed catalog will plug into the same trait once
  the upstream toolchain stabilises.
- `MetadataCatalog::list_files` is fallible from the first public catalog
  surface, so future object-store/Iceberg enumeration errors can propagate
  as `StorageError` instead of being collapsed into an empty file list.
- `StorageError::Backend` now carries a boxed `#[source]` error and a
  `StorageError::backend(err)` constructor, so backend implementations can
  preserve the original error chain instead of stringifying it.

### Deprecated

- `PlaceholderCatalog` is retained for source compatibility but is
  deprecated; prefer
  [`catalog::InMemoryMetadataCatalog`](catalog/struct.InMemoryMetadataCatalog.html)
  for new code. It will be removed in `0.2`.

## [0.1.0] - 2026-05-04

### Added

- Initial scaffolding release. Ships `PlaceholderCatalog` and the
  `MetadataCatalog`-shaped surface plan documented in the crate root.
- The implementation is gated on verifying the
  `iceberg-rust` + `datafusion-iceberg` + `ballista` combination on a
  recent stable Rust toolchain. The crate is published early so
  downstream consumers can pin a version and depend on the planned
  `MetadataCatalog` trait once it lands.
