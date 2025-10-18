//! tile_data.rs
//!
//! Defines the smallest unit of procedural data stored within a Chunk.
//!
//! This data structure holds the specific runtime state and physical characteristics
//! for a single tile at a given (x, y) coordinate.

use serde::{Deserialize, Serialize};

// This type is defined in the sister module `tile_type.rs`.
use super::tile_type::TileType;

/// The canonical data structure for a single tile.
///
/// This struct must be cheap to clone and serialize, as millions of instances
/// will be created and processed during generation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileData {
    /// The fundamental type of the tile (e.g., Land, Water, Void).
    pub tile_type: TileType,
    /// Height or elevation value, critical for noise-based generation.
    pub height_value: f32,
    /// An auxiliary data value, typically used for temperature, moisture, or flow.
    pub aux_value: f32,
    /// A bitmask or ID used for connecting adjacent tiles (e.g., 47-bit mask for terrains).
    pub structure_id: u32,
    /// A general purpose flag used to track states like 'is_on_fire' or 'is_protected'.
    pub flags: u8,
}

impl Default for TileData {
    /// Provides a default, uninitialized state for a new TileData instance.
    fn default() -> Self {
        TileData {
            tile_type: TileType::Void, // Defaults to the empty state.
            height_value: 0.0,
            aux_value: 0.0,
            structure_id: 0,
            flags: 0,
        }
    }
}

impl TileData {
    /// Checks if the tile is considered solid or non-traversable.
    /// This method is now exhaustive, covering all `TileType` variants.
    pub fn is_solid(&self) -> bool {
        match self.tile_type {
            // Only Land and Structure are currently considered solid/non-air for pathfinding.
            TileType::Land | TileType::Structure => true,
            // Handle all other types (Void, Water, Atmospheric, Boundary, CustomX) as non-solid.
            _ => false,
        }
    }

    /// Sets a specific bit flag.
    pub fn set_flag(&mut self, flag_index: u8) {
        if flag_index < 8 {
            // FIX: Removed unnecessary parentheses, resolving the previous warning.
            self.flags |= 1 << flag_index;
        }
    }

    /// Checks if a specific bit flag is set.
    pub fn check_flag(&self, flag_index: u8) -> bool {
        if flag_index < 8 {
            (self.flags & (1 << flag_index)) != 0
        } else {
            false
        }
    }
    
    /// Clears a specific bit flag.
    pub fn clear_flag(&mut self, flag_index: u8) {
        if flag_index < 8 {
            self.flags &= !(1 << flag_index);
        }
    }
}
