# Architecture (v0.2 — 2D)

```text
aetherion-cli
  menu   → interactive number/letter menu (default)
  build  → cargo build -p aetherion --features godot
  deploy / install → copy cdylib + .gdextension (+ gift API script)
  launch → Godot --path examples/godot_demo  (showcase)
  g path → scaffold / open your game with AetherionEngine + API bible

templates/gift/aetherion_engine_api.gd
  → copied onto AetherionEngine on every install (API knowledge)

examples/gift_game     tracked starter (no DLL in git)
examples/godot_demo    full staged callable tour
```

## Crates

| Crate | Role |
|-------|------|
| `aetherion` | 2D core types + optional GDExtension |
| `aetherion-cli` | Operator surface (build → deploy → launch) |

## Host apply (2D only)

```text
generate_region (Rayon-parallel chunks)
  → TileMap:    host_tilemap (procedural atlas + set_cell_ex)
  → MultiMesh:  host_multimesh (colored quads, Plan-B-lite 2D)
bind_tilemap / bind_multimesh / set_prefer_multimesh
flood_million → 16×16×64² = 1,048,576 tiles via MultiMesh2D
bench_4m_cpu  → 32×32×64² = 4,194,304 tiles (CPU only)
```

## Scope

**2D by design.** TileMap and MultiMeshInstance2D are the supported hosts.  
3D mesh/atlas renderers are **not** part of this product — fork if you need them.
