# Contributing to Aetherion

Thanks for helping. Keep the scope sharp: **Rust owns generation; Godot owns presentation.**

## Setup

1. Install Rust matching `rust-toolchain.toml` (currently 1.87).
2. Optionally install **Godot 4.2+** and set `GODOT_BIN` to the executable.
3. From the repo root:

```bash
cargo test --workspace
cargo run -p aetherion-cli -- doctor
# with Godot:
cargo run -p aetherion-cli -- smoke
```

## Guidelines

- No absolute machine paths (`C:\…`) in code; use env/config and paths relative to the workspace.
- Do **not** commit Godot editor binaries, `.dll` / `.so` / `.dylib` build products, or `.godot/` caches.
- Prefer small, documented modules over monorepo dumps from SSXL-ext / old Aetherion.
- Scale numbers in README/BENCHMARKS must be **measured on this tree**, not copied from SSXL-ext alone.
- `cargo fmt` and `cargo clippy --workspace -- -D warnings` should pass.

## Pull requests

1. Branch from `main`.
2. Keep PRs focused (one feature or fix).
3. Update `CHANGELOG.md` under “Unreleased” or the next version section.
4. If you touch the GDExtension API, note Godot version tested.

## Architecture pointers

- `docs/ARCHITECTURE.md` — crate/host flow  
- `docs/LINEAGE.md` — relationship to U8.4 and SSXL-ext  
- Old code lives only on `archive/u8.4-pre-greenfield` for reference  
