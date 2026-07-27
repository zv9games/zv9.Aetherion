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

## Later (not in v0.1)

- Bulk TileMap apply / streaming conductor
- Plan B mesh renderer (SSXL-ext `SSXLRenderer` lineage)
- Headless / automated Godot tests
