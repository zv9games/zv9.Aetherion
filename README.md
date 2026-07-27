# Aetherion

**Rust procedural generation core for Godot 4** via GDExtension.

Greenfield rebuild for the open-source community.  
Historic U8.4 multi-crate tree: branch/tag [`archive/u8.4-pre-greenfield`](https://github.com/zv9games/aetherion/tree/archive/u8.4-pre-greenfield).  
Scale confirmation lineage: [`ssxl-ext`](https://github.com/zv9games/ssxl-ext) tag `archive/confirmation-record`.

> Rust owns generation. Godot owns presentation.

## Status

| Milestone | State |
|-----------|--------|
| Workspace + CLI scaffold | **yes** |
| GDExtension loads in Godot | *in progress* |
| Bulk tile / scale path | planned |
| Published re-measured benches | planned |

## Requirements

- Rust (see `rust-toolchain.toml`)
- **Godot 4.2+** installed separately (not vendored)
- Windows x64 primary; Linux/macOS libraries mapped in `.gdextension`

## Quick start

```bash
git clone https://github.com/zv9games/aetherion.git
cd aetherion

# No Godot needed:
cargo test -p aetherion
cargo run -p aetherion-cli -- doctor

# With Godot 4.x:
# Windows PowerShell:
$env:GODOT_BIN = "C:\path\to\Godot_v4.x_win64.exe"
cargo run -p aetherion-cli -- run
```

CLI binary name: **`aetherion-cli`**.

| Command | Action |
|---------|--------|
| `aetherion-cli doctor` | Version, health, Godot path |
| `aetherion-cli build` | `cargo build -p aetherion --features godot --release` |
| `aetherion-cli deploy` | Copy cdylib into `examples/godot_demo` |
| `aetherion-cli run` | build + deploy + launch Godot |

## Layout

```text
crates/aetherion       engine (rlib + cdylib, feature godot)
crates/aetherion-cli   operator CLI (build → deploy → run)
examples/godot_demo    minimal Godot project
docs/                  lineage, architecture, benchmarks
```

## License

Dual-licensed under **MIT** OR **Apache-2.0**.

## Lineage

See [docs/LINEAGE.md](docs/LINEAGE.md).
