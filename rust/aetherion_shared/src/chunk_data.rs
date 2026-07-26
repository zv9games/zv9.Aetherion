//! Canonical data structure for a single, dimension-agnostic chunk of procedural data.

use serde::{Serialize, Deserialize};
use std::time::SystemTime;

// Import types from other modules in aetherion_shared
use crate::grid_bounds::GridBounds;
use crate::tile_data::TileData; 

pub mod system_time_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let nanos = time.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO).as_nanos() as u64;
        serializer.serialize_u64(nanos)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + Duration::from_nanos(nanos))
    }
}

// Import Vec2i from the dedicated aetherion_math crate (External Dependency)
use aetherion_math::Vec2i; 

// ❌ Removed: use serde_big_array::BigArray;

// --- CONSTANTS ---

/// The canonical size for all chunks in the Aetherion Engine (32x32 tiles).
pub const CHUNK_SIZE: u32 = 32;
/// The total number of tiles in a single chunk (32 * 32 = 1024).
const TILE_COUNT: usize = (CHUNK_SIZE * CHUNK_SIZE) as usize; 

// --- STRUCT DEFINITION ---
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct ChunkData {
    pub id: u64,
    pub bounds: GridBounds,
    /// 🚀 SSXL Update: Changed from a fixed-size array to a dynamic Vec<TileData> 
    /// for maximum flexibility, serialization simplicity, and reduced code complexity.
    // ❌ Removed: #[serde(with = "BigArray")]
    pub tiles: Vec<TileData>,
    pub dimension_tag: String,
    #[serde(with = "system_time_serde")]
    pub generated_at: SystemTime,
}

// --- IMPLEMENTATION ---

impl ChunkData {
    pub const SIZE: u32 = CHUNK_SIZE;

    /// Creates a new, empty ChunkData instance initialized with default data.
    pub fn new(id: u64, bounds: GridBounds, dimension_tag: String) -> Self {
        // 🚀 Initialize Vec<TileData>
        let tiles = vec![TileData::default(); TILE_COUNT];
        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag,
            generated_at: SystemTime::now(),
        }
    }
    
    /// Creates a new ChunkData instance using only the chunk coordinates.
    pub fn new_at_coords(chunk_coords: Vec2i) -> Self {
        let chunk_size_i64 = CHUNK_SIZE as i64;
        
        // Convert chunk coordinates to world-space grid coordinates (i64 for GridBounds)
        let min_x = chunk_coords.x as i64 * chunk_size_i64;
        let min_y = chunk_coords.y as i64 * chunk_size_i64;
        let max_x = min_x + chunk_size_i64 - 1;
        let max_y = min_y + chunk_size_i64 - 1;

        let bounds = GridBounds::new(min_x, min_y, max_x, max_y);
        
        // NOTE: In a final system, the ID should be derived via robust hashing.
        let id = chunk_coords.x as u64 ^ chunk_coords.y as u64; 
        // 🚀 Initialize Vec<TileData>
        let tiles = vec![TileData::default(); TILE_COUNT];

        ChunkData {
            id,
            bounds,
            tiles,
            dimension_tag: "Default".to_string(),
            generated_at: SystemTime::now(),
        }
    }

    /// Converts local (x, y) coordinates to a flattened array index.
    #[inline(always)]
    fn coord_to_index(x: u32, y: u32) -> Option<usize> {
        if x < Self::SIZE && y < Self::SIZE {
            Some((y * Self::SIZE + x) as usize)
        } else {
            None
        }
    }

    /// Returns the data for a tile at the given local (x, y) coordinates.
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&TileData> {
        Self::coord_to_index(x, y).map(|index| {
            &self.tiles[index]
        })
    }
    
    /// **CRITICAL FIX for CA Generator:** Replaces the chunk's tile data with a new set of tiles.
    /// The function is updated for the new `Vec<TileData>` structure.
    pub fn insert_tiles(&mut self, tiles_vec: Vec<TileData>) {
        if tiles_vec.len() == TILE_COUNT {
            // 🚀 SSXL Update: Efficiently replace the entire vector data.
            self.tiles = tiles_vec;
        } else {
            panic!(
                "Tile vector size mismatch for chunk {:?}: Expected {} but got {}", 
                self.bounds, 
                TILE_COUNT, 
                tiles_vec.len()
            );
        }
    }
    
    /// Returns a mutable reference to the tile data at the given local (x, y) coordinates.
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut TileData> {
        Self::coord_to_index(x, y).map(|index| {
            &mut self.tiles[index]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile_type::TileType;

    /// Tests the coordinate flattening function for standard and edge cases.
    #[test]
    fn test_coord_to_index() {
        // Chunk size is 32 (CHUNK_SIZE)
        
        // 1. Basic check (0, 0)
        assert_eq!(ChunkData::coord_to_index(0, 0), Some(0));

        // 2. Middle point check
        // (Y=16 * SIZE=32) + X=16 = 512 + 16 = 528
        assert_eq!(ChunkData::coord_to_index(16, 16), Some(528));

        // 3. Max boundary check (31, 31)
        // (Y=31 * SIZE=32) + X=31 = 992 + 31 = 1023 (TILE_COUNT - 1)
        assert_eq!(ChunkData::coord_to_index(31, 31), Some(1023));

        // 4. Out-of-bounds check (size is 32, so 32 is out)
        assert_eq!(ChunkData::coord_to_index(32, 0), None);
        assert_eq!(ChunkData::coord_to_index(0, 32), None);
        assert_eq!(ChunkData::coord_to_index(33, 33), None);
    }
    
    /// Tests that the new ChunkData initializes correctly and that insert_tiles works with Vec.
    #[test]
    fn test_chunk_data_init_and_insert() {
        let bounds = GridBounds::new(0, 0, 31, 31);
        let mut chunk = ChunkData::new(1, bounds, "test_dim".to_string());
        
        // Check initialization size
        assert_eq!(chunk.tiles.len(), TILE_COUNT);
        assert_eq!(chunk.tiles[0].tile_type, TileType::default());

        // Create a new Vec of tiles for insertion
        let mut new_tiles = vec![TileData::default(); TILE_COUNT];
        
        // Set a tile to a unique type for verification
        let index_to_change = ChunkData::coord_to_index(1, 1).unwrap();
        new_tiles[index_to_change].tile_type = TileType::Rock;
        
        // Insert (replace) the tiles
        chunk.insert_tiles(new_tiles);

        // Verify the change
        assert_eq!(chunk.get_tile(1, 1).unwrap().tile_type, TileType::Rock);
    }
}