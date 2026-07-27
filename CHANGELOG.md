# Changelog

## Unreleased

## [0.2.0] — 2026-07-27

**2D by design.** TileMap + MultiMesh2D host paths. 3D is out of scope — fork if you need it.

### Added
- Interactive CLI menu (default `aetherion-cli` / `cargo run -p aetherion-cli`)
- Menu **g**: scaffold project, install plugin + extension, open Godot **editor**
- Menu **l**: repo API showcase demo (`examples/godot_demo`)
- Auto-find `Godot*.exe` in **repo root** (n00b path) or `GODOT_BIN`
- `install` / `deploy --to` — hook Aetherion into any Godot project
- **API bible** `templates/gift/aetherion_engine_api.gd` attached to **AetherionEngine** on gift install
- Tracked gift project `examples/gift_game` (scene + bible, no DLL)
- README + n00b bible: install tools → clone → build → drop Godot → press **g**
- Restored README poster (`.assets/godot_aetherion.jpg`)
- crates.io packages: `aetherion`, `aetherion-cli`

### Fixed
- Gift script: use `get_viewport().get_visible_rect()` (`AetherionEngine` is a bare `Node`)
- Install/open uses `--editor` so projects without a run-only main scene still open
- Scaffold creates `project.godot` + starter scene when folder is empty

### Scope
- Official product is **2D** procedural gen for Godot 4
- 3D mesh/atlas “Plan B” is **not** a roadmap item — fork for 3D experiments

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
- Packaged binary releases of Godot (install your own)
