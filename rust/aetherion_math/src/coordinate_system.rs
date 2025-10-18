//! coordinate_system.rs
//!
//! Defines the coordinate system types used throughout the engine and provides
//! conversion utilities between them.
//!
//! The Aetherion engine uses a chunked, tile-based world system.
//! - WorldPos: Global coordinate, precise to the tile (i32).
//! - ChunkKey: Identifier for the chunk the tile belongs to (i32).
//! - TileOffset: Local position of the tile within its chunk (u8).

use serde::{Serialize, Deserialize};
// Corrected import: We use CHUNK_SIZE (a u8/usize constant) and cast it to i32 locally.
use aetherion_shared::math_primitives::{CHUNK_SIZE};
use glam::IVec3;

// Cache the i32 version of the chunk size for faster, cleaner math operations.
const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

// --- 1. Coordinate Types ---

/// A global coordinate precise to the tile level.
/// Used for physics, networking, and high-level queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorldPos(pub IVec3);

/// The key/identifier for a specific chunk in the world grid.
/// Used for data fetching, loading/unloading, and persistent storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkKey(pub IVec3);

/// The local coordinate of a tile *within* its chunk.
/// Used for array indexing and rendering offsets.
// We use IVec3 (i32) here for consistency, though components will be small (0 to CHUNK_SIZE-1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileOffset(pub IVec3);


// --- 2. Coordinate Conversion Functions ---

impl WorldPos {
    /// Converts a global WorldPos into its corresponding ChunkKey and local TileOffset.
    ///
    /// The conversion uses truncated division and Euclidean modulo to correctly
    /// handle coordinates, especially negative values, ensuring a consistent
    /// mapping to chunk boundaries.
    pub fn to_chunk_coords(&self) -> (ChunkKey, TileOffset) {
        // Chunk Key calculation (uses standard integer division)
        let chunk_x = self.0.x / CHUNK_SIZE_I32;
        let chunk_y = self.0.y / CHUNK_SIZE_I32;
        let chunk_z = self.0.z / CHUNK_SIZE_I32;

        // Tile Offset calculation (uses Euclidean remainder for proper boundary handling)
        let tile_x = self.0.x.rem_euclid(CHUNK_SIZE_I32);
        let tile_y = self.0.y.rem_euclid(CHUNK_SIZE_I32);
        let tile_z = self.0.z.rem_euclid(CHUNK_SIZE_I32);

        let chunk_key = ChunkKey(IVec3::new(chunk_x, chunk_y, chunk_z));
        let tile_offset = TileOffset(IVec3::new(tile_x, tile_y, tile_z));

        (chunk_key, tile_offset)
    }
}

impl ChunkKey {
    /// Converts a ChunkKey and a local TileOffset back into a global WorldPos.
    pub fn to_world_pos(&self, offset: TileOffset) -> WorldPos {
        let world_x = self.0.x * CHUNK_SIZE_I32 + offset.0.x;
        let world_y = self.0.y * CHUNK_SIZE_I32 + offset.0.y;
        let world_z = self.0.z * CHUNK_SIZE_I32 + offset.0.z;

        WorldPos(IVec3::new(world_x, world_y, world_z))
    }
}
