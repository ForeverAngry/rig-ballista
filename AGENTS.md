# AGENTS.md

Guidance for AI coding agents working in `rig-ballista`.

## Project

Apache Ballista + DataFusion + Iceberg companion crate for
[`rig-compose`](https://crates.io/crates/rig-compose).

**Status:** scaffolding. Ships `PlaceholderCatalog` until the
`iceberg-rust` + `datafusion-iceberg` + `ballista` combination is verified
on a recent stable Rust toolchain. The publish exists so downstream
consumers can pin a version and depend on the planned `MetadataCatalog`
trait when it lands.

## Rules

- Rust 2024, MSRV 1.88 (placeholder phase). The eventual integration will
  bump MSRV to whatever Iceberg/Ballista require, shipped as `feat!:`.
- Errors: `thiserror` enums; return `Result<_, _>`.
- No `unwrap`/`expect`/`panic!`/`todo!`/`unimplemented!`/`dbg!`/indexing
  in library code. Allowed in `#[cfg(test)]`.
- Document new `pub` items with `///` rustdoc.
- Keep Iceberg/Ballista/DataFusion types **out** of the public surface.
  Expose them through a `MetadataCatalog`-shaped trait so downstream
  agents see only `rig-compose` types.

## Validation

```sh
just check
# fmt + clippy --all-features + test --all-features + rustdoc strict
```

## Scope

Do not pull in real Iceberg/Ballista deps until the throwaway
verification crate confirms the combination compiles cleanly on stable.
Update [README.md](README.md) and [CHANGELOG.md](CHANGELOG.md) for
user-visible changes.
