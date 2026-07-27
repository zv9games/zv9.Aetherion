# Aetherion

<p align="center">
  <img src="./.assets/godot_aetherion.jpg" alt="Godot Aetherion Poster" width="600"/>
</p>

<p align="center">
  <strong>2D procedural generation for Godot 4</strong> · GDExtension · open source
</p>

<p align="center">
  <a href="https://github.com/zv9games/aetherion/actions/workflows/ci.yml"><img src="https://github.com/zv9games/aetherion/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/aetherion"><img src="https://img.shields.io/crates/v/aetherion.svg" alt="crates.io"></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg" alt="License"></a>
</p>

```text
╔═══════════════════════════════════════════════════════════════════════════╗
║  AETHERION — v0.2.0 · 2D by design                                        ║
║                                                                           ║
║  Rust owns generation. Godot owns presentation.                           ║
║  Build the CLI. Drop Godot in the repo root. Press g. Make a game.        ║
║  Want 3D? Fork it.                                                        ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

> New here? Read the full walkthrough: **[docs/NOOB_MANUAL.md](docs/NOOB_MANUAL.md)** (the n00b bible).

---

## 5-minute gift path (Windows)

### 1. Install tools

| Tool | Why | Link |
|------|-----|------|
| **Git** | clone the source | [git-scm.com](https://git-scm.com) |
| **Rust** | compile Aetherion | [rustup.rs](https://rustup.rs) — install, then **close and reopen** the terminal |
| **VS Build Tools** (Windows) | C++ linker for Rust | “Desktop development with C++” workload |
| **Godot 4 stable** | the game editor | [godotengine.org/download](https://godotengine.org/download) — standard **Windows 64-bit** `.exe` |

### 2. Get the source

```powershell
git clone https://github.com/zv9games/aetherion.git
cd aetherion
```

### 3. Drop Godot into the repo root

Copy the official stable binary **into this folder** (next to `Cargo.toml`):

```text
aetherion/
  Cargo.toml
  Godot_v4.7.1-stable_win64.exe    ← put it here (name may vary by version)
  crates/
  ...
```

The CLI auto-finds any `Godot*.exe` in the repo root.  
(Optional: set `$env:GODOT_BIN = "C:\full\path\to\Godot.exe"` instead.)

**Do not commit Godot** — it is gitignored on purpose.

### 4. Compile the CLI

```powershell
cargo build -p aetherion-cli
```

Binary lands at:

```text
target\debug\aetherion-cli.exe
```

(Faster later: `cargo build -p aetherion-cli --release` → `target\release\aetherion-cli.exe`.)

### 5. Run the menu → press **g** → make a game

```powershell
.\target\debug\aetherion-cli.exe
```

Or:

```powershell
cargo run -p aetherion-cli
```

```text
  1  d    doctor
  2  b    build
  3  l    launch demo
  4  g    launch godot + aetherion  ★ make a game
  5  s    smoke
  6  f    bench 4M
  7  t    bench 10M
  0  q    quit
     h    help
```

Type **`g`** (or **`4`**) and press Enter.

- Empty path → creates `godot_save/aetherion-game` for you  
- Builds + installs `aetherion.dll` + `aetherion.gdextension`  
- Attaches **`aetherion_engine_api.gd`** to the **AetherionEngine** node (API bible)  
- Opens the **Godot editor**  

Hit **Play (F5)**: the script on AetherionEngine demos every callable.  
Edit that script — that is where API knowledge lives for new users.

**API showcase (every callable):** menu **`l`** / **`3`** — see [examples/godot_demo](examples/godot_demo).

---

## What you get

| Capability | Status |
|------------|--------|
| **2D** GDExtension node **`AetherionEngine`** | yes |
| Parallel region gen (Rayon) | yes |
| TileMap + MultiMesh2D host apply | yes — up to **`flood_10m` (~10.24M)** |
| Operator CLI menu | yes — doctor / build / demo / **make a game** |
| Scaffold + API bible on engine node | yes — menu **g** + `templates/gift/` |
| Tracked gift project | yes — `examples/gift_game` |
| 3D mesh renderer | **out of scope** — fork if you need 3D |

## Layout

```text
crates/aetherion            engine (rlib + cdylib, feature godot)
crates/aetherion-cli        operator CLI  →  target/*/aetherion-cli.exe
templates/gift/             API bible source (aetherion_engine_api.gd)
examples/gift_game          tracked starter project (no DLL in git)
examples/godot_demo         full staged showcase
godot_save/                 local games only (gitignored)
docs/NOOB_MANUAL.md         n00b bible
.assets/                    poster art
```

## Install (crates.io)

```bash
cargo add aetherion
cargo add aetherion --features godot   # GDExtension bindings

cargo install aetherion-cli
# or from git:
# cargo install --git https://github.com/zv9games/aetherion aetherion-cli
```

Godot is still separate. Use the CLI menu (**g**) or `install` to drop the DLL + extension into a project.

## CLI cheat sheet

```powershell
# Menu (recommended)
.\target\debug\aetherion-cli.exe

# Same, via cargo
cargo run -p aetherion-cli

# Direct commands
cargo run -p aetherion-cli -- doctor
cargo run -p aetherion-cli -- launch          # demo
cargo run -p aetherion-cli -- install PATH --open   # your game
cargo run -p aetherion-cli -- smoke
cargo run -p aetherion-cli --release -- bench10m
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

See [docs/BENCHMARKS.md](docs/BENCHMARKS.md). On a Windows dev box (release CLI):

- **~4.19M** tiles generated in a few **ms** (CPU, Rayon)
- **~10.24M** flood path in the demo (needs RAM for host apply)

## Requirements

- **Rust** — see `rust-toolchain.toml` (currently **1.87**)
- **Godot 4.2+** (4.5–4.7 tested) — drop into repo root or set `GODOT_BIN`
- Windows x64 primary; Linux/macOS lib names in `.gdextension`

## Lineage

**v0.2.0** — gift path, API bible, operator menu, crates.io.  
**v0.1.0** — first greenfield OSS cut.  
Historic multi-crate tree: [`archive/u8.4-pre-greenfield`](https://github.com/zv9games/aetherion/tree/archive/u8.4-pre-greenfield).  
Scale confirmation: [`ssxl-ext`](https://github.com/zv9games/ssxl-ext) (`archive/confirmation-record`).  
Story: [docs/LINEAGE.md](docs/LINEAGE.md).

## License

Dual-licensed **[MIT](LICENSE-MIT)** OR **[Apache-2.0](LICENSE-APACHE)**.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Keep absolute paths and editor binaries out of git.
