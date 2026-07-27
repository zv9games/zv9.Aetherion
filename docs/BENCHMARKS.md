# Benchmarks

## Prior art (SSXL-ext, not re-run on this tree)

Historical confirmation on `zv9games/ssxl-ext` (tag `archive/confirmation-record`):

- Commits describing ~4,000,000 tiles into Godot (order ~82s–104s class).
- Pipeline: Rust GDExtension bulk path + host apply.

**These numbers are not claimed for greenfield Aetherion until re-measured here.**

## Aetherion greenfield

| Date | Machine | Godot | Metric | Result |
|------|---------|-------|--------|--------|
| 2026-07-27 | Windows (dev) | 4.5.1 | CPU `bench --chunks 16 --size 64` (1,048,576 tiles) | **19 ms** (~55M tiles/s wall) |
| 2026-07-27 | Windows (dev) | 4.5.1 headless | GDExtension load + auto-smoke 1024 tiles + `bench_medium` 262144 tiles | **PASS** (extension Scene init, class `AetherionEngine`) |
| 2026-07-27 | Windows (dev) | 4.5.1 headless | Gen + **TileMap apply** 16 384 cells | gen ~0 ms, **apply ~6 ms** |
| 2026-07-27 | Windows (dev) | 4.5.1 headless | Gen + **MultiMesh apply** 16 384 instances | gen ~0 ms, **apply ~5 ms** |
| 2026-07-27 | Windows (dev) | — | CPU **`bench4m`** 32×32×64² = **4,194,304** tiles (Rayon) | **~2 ms** release |
| 2026-07-27 | Windows (dev) | 4.7 window | Host **`flood_10m`** 50×50×64² = **10,240,000** MultiMesh | gen + apply (see on-screen timer) |

**Tile counts**

| Name | Layout | Tiles |
|------|--------|------:|
| TileMap smoke | 2×2 × 16² | 1,024 |
| Medium MultiMesh | 4×4 × 32² | 16,384 |
| `flood_million` | 16×16 × 64² | 1,048,576 |
| **`flood_10m` / `bench10m`** | **50×50 × 64²** | **10,240,000** |

Pacman-class demos needed ~3.5M; **10.24M** is the public “crazy adventure” ceiling for this demo.

CPU benches are generation only. Host apply = TileMap `set_cell_ex` or MultiMesh instance transforms.
