//! Chunk coordinate and tile buffers (host-agnostic).

use serde::{Deserialize, Serialize};

/// Integer chunk coordinate in a 2D lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    /// X index.
    pub x: i32,
    /// Y index.
    pub y: i32,
}

impl ChunkCoord {
    /// Create a new chunk coordinate.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Dense square chunk of atlas/tile indices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    /// Chunk lattice coordinate.
    pub coord: ChunkCoord,
    /// Edge length in tiles (width = height = size).
    pub size: u32,
    /// Row-major tile indices (`size * size` entries).
    pub tiles: Vec<u16>,
}

impl ChunkData {
    /// Allocate an empty chunk filled with zeros.
    pub fn empty(coord: ChunkCoord, size: u32) -> Self {
        let n = (size as usize).saturating_mul(size as usize);
        Self {
            coord,
            size,
            tiles: vec![0; n],
        }
    }

    /// Number of tiles in this chunk.
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Linear index for local (lx, ly).
    pub fn index(&self, lx: u32, ly: u32) -> Option<usize> {
        if lx >= self.size || ly >= self.size {
            return None;
        }
        Some((ly * self.size + lx) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_roundtrip_fields() {
        let c = ChunkCoord::new(3, -2);
        assert_eq!(c.x, 3);
        assert_eq!(c.y, -2);
    }

    #[test]
    fn empty_chunk_size() {
        let c = ChunkData::empty(ChunkCoord::new(0, 0), 16);
        assert_eq!(c.tile_count(), 256);
    }
}
