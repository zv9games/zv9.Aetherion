//! Chunk coordinate types (host-agnostic).

use serde::{Deserialize, Serialize};

/// Integer chunk coordinate in a 2D lattice (3D reserved later).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_roundtrip_fields() {
        let c = ChunkCoord::new(3, -2);
        assert_eq!(c.x, 3);
        assert_eq!(c.y, -2);
    }
}
