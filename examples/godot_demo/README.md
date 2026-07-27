# Aetherion API showcase

Living tour of **every GDScript-callable** on `AetherionEngine`.

## Run it

From the Aetherion repo root:

```powershell
cargo run -p aetherion-cli -- launch
# menu: l
```

## Node tree

```text
Main (main.gd)                 stage conductor + input
├── World
│   ├── Camera2D               pan / zoom
│   ├── TileMap                bind_tilemap host
│   └── MultiMeshInstance2D    bind_multimesh host
├── AetherionEngine            all API methods
└── UI
    ├── Status                 getters HUD
    └── StageLog               recent method calls
```

## Stages → callables

| # | Stage | Methods |
|---|-------|---------|
| 0 | Boot | `get_version`, `get_ticks` |
| 1 | Bind hosts | `bind_tilemap`, `bind_multimesh`, `set_prefer_multimesh` |
| 2 | TileMap checkerboard | `generate_region` (mode 0) |
| 3 | TileMap noise | `generate_region` (mode 1) |
| 4 | CPU only | `generate_region_cpu` |
| 5 | Medium MultiMesh | `set_prefer_multimesh(true)`, `bench_medium` |
| 6 | ~1M flood | `flood_million` |
| 7 | ~10M flood | `flood_10m` (skipped headless) |
| 8 | CPU 4M | `bench_4m_cpu` |
| 9 | CPU 10M | `bench_10m_cpu` |
| * | After gens | `get_last_tiles`, `get_last_ms`, `get_last_apply_ms`, `get_last_summary`, `get_ticks` |

## Controls

| Key | Action |
|-----|--------|
| WASD / arrows | pan camera |
| mouse wheel | zoom |
| Space | skip stage hold |
| R | restart full tour |
| Esc | quit |

## Copy into your game

Minimal pattern from stage 1–2:

```gdscript
@onready var engine: Node = $AetherionEngine
@onready var tilemap: TileMap = $World/TileMap

func _ready() -> void:
	engine.bind_tilemap(tilemap)
	engine.set_prefer_multimesh(false)
	print(engine.generate_region(0, 0, 4, 4, 32, 1, 42))
	print(engine.get_last_summary())
```
