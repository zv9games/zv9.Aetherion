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

CPU benches are host-side generation only (no TileMap apply yet).
