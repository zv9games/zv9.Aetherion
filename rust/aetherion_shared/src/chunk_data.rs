// aetherion_shared/src/chunk_data.rs
//! Canonical data structure for a single, dimension-agnostic chunk of procedural data.
//!
//! This structure must be Clone, Debug, and fully Serializable/Deserializable.

use serde::{Serialize, Deserialize};

// Placeholder: These types will be defined in other modules of aetherion_shared.
// For now, assume they exist.
use crate::grid_bounds::GridBounds;
use crate::tile_data::TileData;

/// The canonical size for all chunks in the Aetherion Engine.
pub const CHUNK_SIZE: usize = 32;

/// Data payload representing a single chunk in the procedural world.
///
/// This structure is the fundamental unit of storage, processing, and transfer
/// within the engine pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    /// Unique identifier or seed derived from its world position.
    pub id: u64,
    /// The boundary coordinates of this chunk in world space.
    pub bounds: GridBounds,
    /// The actual procedural data stored in a flattened array.
    /// Layout is [CHUNK_SIZE * CHUNK_SIZE] for 2D.
    pub tiles: Vec<TileData>,
    /// Metadata tag for the dimension or layer this chunk belongs to.
    pub dimension_tag: String,
    /// Timestamp of the last generation or modification.
    pub generated_at: u64,
}

impl ChunkData {
    /// Creates a new, empty ChunkData instance initialized with default data.
    ///
    /// This is used primarily during initial pipeline setup or testing.
    pub fn new(id: u64, bounds: GridBounds, dimension_tag: String) -> Self {
        // Initializes a flattened vector of TileData instances.
        let tile_count = CHUNK_SIZE * CHUNK_SIZE;
        let tiles = vec![TileData::default(); tile_count];

        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag,
            generated_at: 0, // Placeholder
        }
    }

    /// Returns the data for a tile at the given local (x, y) coordinates.
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TileData> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            self.tiles.get(y * CHUNK_SIZE + x)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the tile data at the given local (x, y) coordinates.
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut TileData> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            self.tiles.get_mut(y * CHUNK_SIZE + x)
        } else {
            None
        }
    }
}
