# Architecture (v0.1)

```text
aetherion-cli
  build  → cargo build -p aetherion --features godot
  deploy → copy cdylib → examples/godot_demo/
  run    → GODOT_BIN --path examples/godot_demo

examples/godot_demo
  aetherion.gdextension → aetherion.dll / libaetherion.so
  AetherionEngine (Node)  from feature "godot"
```

## Crates

| Crate | Role |
|-------|------|
| `aetherion` | Core types + optional GDExtension |
| `aetherion-cli` | Operator surface (SSXL-ext pipeline, cleaned) |

## Host apply (Phase 3)

```text
AetherionEngine.bind_tilemap(TileMap)
  → ensure_demo_tileset (4-color atlas if missing)
generate_region(...)
  → conductor::run_region_data
  → host_tilemap::apply_chunks_to_tilemap (set_cell_ex)
```

## Later

- Streaming / async conductor workers
- Plan B mesh renderer (SSXL-ext `SSXLRenderer` lineage)
- Larger TileMap bulk APIs if Godot exposes them
