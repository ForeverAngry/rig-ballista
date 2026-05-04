# rig-ballista

[![CI](https://github.com/ForeverAngry/rig-ballista/actions/workflows/ci.yml/badge.svg)](https://github.com/ForeverAngry/rig-ballista/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rig-ballista.svg)](https://crates.io/crates/rig-ballista)
[![docs.rs](https://img.shields.io/docsrs/rig-ballista)](https://docs.rs/rig-ballista)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/rustc-1.88+-orange.svg)](#rust-version)

Apache Ballista + DataFusion + Iceberg companion crate for
[`rig-compose`](https://crates.io/crates/rig-compose).

> **Status: scaffolding.** The implementation is gated on verifying the
> `iceberg-rust` + `datafusion-iceberg` + `ballista` combination compiles
> on a recent stable Rust toolchain. The crate is published early so
> downstream consumers can pin a version and depend on the planned
> `MetadataCatalog` trait once it lands.

## Why a separate crate

- **MSRV isolation.** `iceberg-rust` 0.9 requires rustc 1.92; pinning
  that into rig-compose would force every consumer onto the same
  toolchain.
- **Compile-cost isolation.** Ballista pulls in DataFusion, Arrow, gRPC,
  Parquet — none of which the kernel needs.
- **Boundary discipline.** Ballista is a query-engine boundary, not a
  file-format boundary. Keeping it behind a trait keeps the skill/tool
  surface clean.

## Rust version

The crate targets Rust **1.88** (edition 2024) for the placeholder
surface. The eventual integration will track the Iceberg/Ballista
toolchain floor and ship as a `feat!:` MSRV bump.

## License

Dual-licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual-licensed as above, without any additional terms or conditions.
