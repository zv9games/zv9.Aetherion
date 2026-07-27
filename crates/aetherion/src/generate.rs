//! Lightweight generators (SSXL-ext checkerboard lineage, no heavy deps).
//!
//! Chunk lattice generation is parallelized with Rayon (SSXL-ext “bulldozer” idea).

use crate::chunk::{ChunkCoord, ChunkData};
use rayon::prelude::*;

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
    let tiles = &mut chunk.tiles;
    // Inner loop is sequential per chunk; chunks themselves are parallelized in `generate_region`.
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
            let i = (ly * size + lx) as usize;
            tiles[i] = tile;
        }
    }
    chunk
}

/// Generate a rectangular region of chunks in parallel; returns flat list + total tiles.
pub fn generate_region(
    origin: ChunkCoord,
    chunks_x: u32,
    chunks_y: u32,
    chunk_size: u32,
    mode: FillMode,
    seed: u32,
) -> (Vec<ChunkData>, u64) {
    let jobs: Vec<(i32, i32)> = (0..chunks_y)
        .flat_map(|cy| (0..chunks_x).map(move |cx| (cx as i32, cy as i32)))
        .collect();

    let out: Vec<ChunkData> = jobs
        .into_par_iter()
        .map(|(cx, cy)| {
            let coord = ChunkCoord::new(origin.x + cx, origin.y + cy);
            generate_chunk(coord, chunk_size, mode, seed)
        })
        .collect();

    let tiles: u64 = out.iter().map(|c| c.tile_count() as u64).sum();
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
