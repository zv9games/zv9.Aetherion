//! Defines the canonical set of fundamental types that a Tile can represent.
//!
//! This enum is used by the generation modules to assign meaning to raw noise values,
//! and by the rendering engine to select the correct visual asset.

use serde::{Serialize, Deserialize};

/// The fundamental, physical classification of a tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)] // Ensures compact storage
pub enum TileType {
    /// 0: The default, empty, or uninitialized state. (Non-solid, non-traversable).
    Void = 0,
    /// 1: Represents solid, traversable land or a basic ground surface (e.g., Grass, Dirt, Stone).
    Land = 1,
    /// 2: Represents a water body (e.g., Lake, Ocean, River).
    Water = 2,
    /// 3: Represents a structured object or built environment (e.g., Wall, Floor, Road).
    Structure = 3,
    /// 4: Represents atmospheric elements or height boundaries (e.g., Cloud, Deep Space).
    Atmospheric = 4,
    /// 5: Represents a boundary or special-condition tile that cannot be traversed or modified easily.
    Boundary = 5,
    /// Reserved for future expansion or custom user types.
    Custom1 = 6,
    /// Reserved for future expansion or custom user types.
    Custom2 = 7,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Void
    }
}

impl TileType {
    /// Helper function to check if the tile type indicates a traversable surface.
    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Land | TileType::Structure => true,
            _ => false,
        }
    }

    /// Helper function to check if the tile type is one that typically requires fluid dynamics simulation.
    pub fn is_fluid(&self) -> bool {
        matches!(self, TileType::Water)
    }

    /// Converts the enum variant into its underlying u8 representation.
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    /// Attempts to convert a u8 into a TileType.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TileType::Void),
            1 => Some(TileType::Land),
            2 => Some(TileType::Water),
            3 => Some(TileType::Structure),
            4 => Some(TileType::Atmospheric),
            5 => Some(TileType::Boundary),
            6 => Some(TileType::Custom1),
            7 => Some(TileType::Custom2),
            _ => None,
        }
    }
}
