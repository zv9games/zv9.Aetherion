# Changelog

## Unreleased

### Added
- Interactive CLI menu (default `cargo run -p aetherion-cli`) — pick by number or letter
- `editor` command — plain Godot / Project Manager (no demo); aliases `pm`, `project-manager`
- Menu keys: demo launch (`4`/`l`), plain Godot (`6`/`g`), doctor/build/deploy/smoke/benches
- Restored README poster (`.assets/godot_aetherion.jpg`) from pre-greenfield archive

## [0.1.0] — 2026-07-27

First greenfield open-source release of **Aetherion**.

### Added
- `aetherion` engine crate (`rlib` + `cdylib`) with optional `godot` feature (godot-rust 0.4.5)
- `AetherionEngine` GDExtension node: version/health, timed region generation
- Parallel (Rayon) chunk generators: checkerboard + hash noise
- Host apply paths:
  - **TileMap** with procedural 4-color atlas (`bind_tilemap`)
  - **MultiMeshInstance2D** Plan-B-lite floods (`bind_multimesh`, `flood_million`)
- Operator CLI `aetherion-cli`: `doctor`, `build`, `deploy`, `run`, `smoke`, `bench`, `bench4m`
- Minimal `examples/godot_demo` (Godot 4.2+)
- Docs: LINEAGE, ARCHITECTURE, BENCHMARKS
- CI: check / test / fmt / clippy on Ubuntu
- Dual license MIT OR Apache-2.0

### Lineage
- Historic multi-crate U8.4 tree: tag/branch `archive/u8.4-pre-greenfield`
- SSXL-ext confirmation vault: `zv9games/ssxl-ext` tag `archive/confirmation-record`

### Not in 0.1.0
- Full 3D mesh atlas renderer (SSXL-ext `SSXLRenderer` class)
- Async streaming worker pool beyond Rayon chunk parallel
- Packaged binary releases of Godot (install your own)
