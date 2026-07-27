# Architecture (v0.1)

```text
aetherion-cli
  build  → cargo build -p aetherion --features godot
  deploy → copy cdylib → examples/godot_demo/
  menu   → interactive number/letter menu (default)
  launch → GODOT_BIN --path examples/godot_demo
  editor → GODOT_BIN (plain Project Manager, no demo)

examples/godot_demo
  aetherion.gdextension → aetherion.dll / libaetherion.so
  AetherionEngine (Node)  from feature "godot"
```

## Crates

| Crate | Role |
|-------|------|
| `aetherion` | Core types + optional GDExtension |
| `aetherion-cli` | Operator surface (SSXL-ext pipeline, cleaned) |

## Host apply

```text
generate_region (Rayon-parallel chunks)
  → TileMap:    host_tilemap (procedural atlas + set_cell_ex)
  → MultiMesh:  host_multimesh (colored quads, Plan-B-lite)
bind_tilemap / bind_multimesh / set_prefer_multimesh
flood_million → 16×16×64² = 1,048,576 tiles via MultiMesh
bench_4m_cpu  → 32×32×64² = 4,194,304 tiles (CPU only)
```

## Later

- Streaming / async conductor workers  
- Full 3D Plan B mesh renderer (SSXL-ext `SSXLRenderer` lineage)
