//! Lightweight generators (SSXL-ext checkerboard lineage, no heavy deps).

use crate::chunk::{ChunkCoord, ChunkData};

/// Fill mode for demo generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMode {
    /// Classic checkerboard (debug atlas pairing).
    Checkerboard,
    /// Value-noise-ish stripes (cheap hash).
    HashNoise,
}

/// Generate one chunk of tiles.
pub fn generate_chunk(coord: ChunkCoord, size: u32, mode: FillMode, seed: u32) -> ChunkData {
    let mut chunk = ChunkData::empty(coord, size);
    let s = size as i32;
    for ly in 0..size {
        for lx in 0..size {
            let wx = coord.x * s + lx as i32;
            let wy = coord.y * s + ly as i32;
            let tile = match mode {
                FillMode::Checkerboard => {
                    if ((wx + wy) & 1) == 0 {
                        1
                    } else {
                        2
                    }
                }
                FillMode::HashNoise => {
                    let h = hash2(wx, wy, seed);
                    1 + (h % 4) as u16
                }
            };
            if let Some(i) = chunk.index(lx, ly) {
                chunk.tiles[i] = tile;
            }
        }
    }
    chunk
}

/// Generate a rectangular region of chunks; returns flat list + total tiles.
pub fn generate_region(
    origin: ChunkCoord,
    chunks_x: u32,
    chunks_y: u32,
    chunk_size: u32,
    mode: FillMode,
    seed: u32,
) -> (Vec<ChunkData>, u64) {
    let mut out = Vec::with_capacity((chunks_x * chunks_y) as usize);
    let mut tiles: u64 = 0;
    for cy in 0..chunks_y {
        for cx in 0..chunks_x {
            let coord = ChunkCoord::new(origin.x + cx as i32, origin.y + cy as i32);
            let chunk = generate_chunk(coord, chunk_size, mode, seed);
            tiles += chunk.tile_count() as u64;
            out.push(chunk);
        }
    }
    (out, tiles)
}

fn hash2(x: i32, y: i32, seed: u32) -> u32 {
    let mut n = (x as u32)
        .wrapping_mul(374761393)
        .wrapping_add((y as u32).wrapping_mul(668265263))
        .wrapping_add(seed.wrapping_mul(362437));
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n ^ (n >> 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkerboard_not_all_zero() {
        let c = generate_chunk(ChunkCoord::new(0, 0), 8, FillMode::Checkerboard, 1);
        assert!(c.tiles.iter().any(|&t| t == 1));
        assert!(c.tiles.iter().any(|&t| t == 2));
    }

    #[test]
    fn region_tile_count() {
        let (_chunks, n) = generate_region(ChunkCoord::new(0, 0), 2, 2, 16, FillMode::HashNoise, 7);
        assert_eq!(n, 2 * 2 * 16 * 16);
    }
}
