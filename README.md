# Aetherion

[![CI](https://github.com/zv9games/aetherion/actions/workflows/ci.yml/badge.svg)](https://github.com/zv9games/aetherion/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

**Rust procedural generation for Godot 4** via GDExtension.

> Rust owns generation. Godot owns presentation.

**v0.1.0** — greenfield open-source release.  
Historic multi-crate tree: [`archive/u8.4-pre-greenfield`](https://github.com/zv9games/aetherion/tree/archive/u8.4-pre-greenfield).  
Scale confirmation lineage: [`ssxl-ext`](https://github.com/zv9games/ssxl-ext) (`archive/confirmation-record`).

## Features

| Capability | Status |
|------------|--------|
| GDExtension loads in Godot 4.5 | yes — `AetherionEngine` |
| Parallel region generation (Rayon) | yes |
| TileMap host apply (procedural atlas) | yes |
| MultiMesh large flood (Plan-B-lite) | yes — `flood_million` (~1M), **`flood_10m` (~10.24M)** |
| CPU ~4.19M / ~10.24M tiles | yes — `bench4m` / **`bench10m`** |
| On-screen stage timer + hold | yes — demo HUD |
| Operator CLI (build → deploy → run) | yes |
| Full 3D mesh Plan B (SSXL-ext renderer) | future |

## Requirements

- **Rust** (see `rust-toolchain.toml`, currently 1.87)
- **Godot 4.2+** installed separately (not vendored in this repo)
- Windows x64 primary; Linux/macOS lib names in `.gdextension`

## Quick start

**New to Godot?** Use the one-page walkthrough: **[docs/NOOB_MANUAL.md](docs/NOOB_MANUAL.md)**  
(install Godot → set `GODOT_BIN` → `cargo run -p aetherion-cli -- run`).

```bash
git clone https://github.com/zv9games/aetherion.git
cd aetherion

# No Godot required:
cargo test --workspace
cargo run -p aetherion-cli -- doctor
cargo run -p aetherion-cli --release -- bench4m

# With Godot 4.x (PowerShell example):
$env:GODOT_BIN = "C:\path\to\Godot_v4.x_win64.exe"
cargo run -p aetherion-cli -- smoke    # headless
cargo run -p aetherion-cli -- launch   # open Godot in Aetherion environment
# aliases: run, godot
```

### CLI (`aetherion-cli`)

| Command | Action |
|---------|--------|
| `doctor` | Version, health, demo path, `GODOT_BIN` |
| `build` | Release cdylib with `--features godot` |
| `deploy` | Copy library into `examples/godot_demo` + extension list |
| **`launch`** | **Build + deploy + open Godot in Aetherion env** (aliases: `run`, `godot`) |
| `launch --no-build` | Deploy existing DLL + open Godot (skip cargo) |
| `smoke` | build + deploy + headless quit-after |
| `bench` | CPU region timing |
| `bench4m` | CPU ~4.19M tiles |
| `bench10m` | CPU ~10.24M tiles (adventure-class headroom) |

## Layout

```text
crates/aetherion       engine (rlib + cdylib, feature godot)
crates/aetherion-cli   operator CLI
examples/godot_demo    minimal Godot 4 project
docs/                  lineage, architecture, benchmarks
```

## Godot API (extension)

Node class: **`AetherionEngine`**

| Method | Purpose |
|--------|---------|
| `bind_tilemap(TileMap)` | Host apply via TileMap |
| `bind_multimesh(MultiMeshInstance2D)` | Fast large floods |
| `set_prefer_multimesh(bool)` | Prefer MultiMesh when both bound |
| `generate_region(...)` | Gen + apply |
| `generate_region_cpu(...)` | Gen only |
| `bench_medium()` / `flood_million()` / `bench_4m_cpu()` | Built-in benches |
| `get_last_summary()` / `get_last_ms()` / `get_last_apply_ms()` | Metrics |

## Benchmarks (indicative)

See [docs/BENCHMARKS.md](docs/BENCHMARKS.md). On a Windows dev machine (release CLI):

- **4,194,304** tiles generated in ~**2 ms** (CPU, Rayon)
- GDExtension load + TileMap/MultiMesh apply verified under Godot **4.5.1** headless

Hardware varies — re-run `bench` / `bench4m` / `smoke` on yours.

## License

Dual-licensed under **[MIT](LICENSE-MIT)** OR **[Apache-2.0](LICENSE-APACHE)**.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Please keep absolute paths and editor binaries out of git.

## Lineage

[docs/LINEAGE.md](docs/LINEAGE.md) — how U8.4 and SSXL-ext feed this tree without shipping their baggage.
