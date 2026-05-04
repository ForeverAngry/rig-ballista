# Changelog

All notable changes to `rig-ballista` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Versions are managed automatically by [release-plz](https://release-plz.dev/)
from [Conventional Commits](https://www.conventionalcommits.org/).

## [Unreleased]

## [0.1.0] - Unreleased

### Added

- Initial scaffolding release. Ships `PlaceholderCatalog` and the
  `MetadataCatalog`-shaped surface plan documented in the crate root.
- The implementation is gated on verifying the
  `iceberg-rust` + `datafusion-iceberg` + `ballista` combination on a
  recent stable Rust toolchain. The crate is published early so
  downstream consumers can pin a version and depend on the planned
  `MetadataCatalog` trait once it lands.
