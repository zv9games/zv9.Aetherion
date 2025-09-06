//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/data/tile.rs

use serde::{Deserialize, Serialize};
use super::vector::SerializableVector2i;

/// Metadata for a single tile in the map.
/// Used for procedural generation, chunk streaming, and Godot tile placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileInfo {
    /// ID of the source tileset or layer.
    pub source_id: i32,

    /// Coordinates in the atlas (e.g. tile index).
    pub atlas_coords: SerializableVector2i,

    /// Alternate ID for visual variation or animation.
    pub alternate_id: i32,

    /// Rotation in 90° steps (0–3).
    pub rotation: u8,

    /// Layer index for multi-layer maps.
    pub layer: u8,

    /// Bitmask for collision, visibility, or custom flags.
    pub flags: u32,
}

/// Bitmask flags for tile behavior and rendering.
/// Combine using bitwise OR.
pub mod tile_flags {
    pub const COLLIDABLE: u32     = 0b00001;
    pub const VISIBLE: u32        = 0b00010;
    pub const INTERACTIVE: u32    = 0b00100;
    pub const EMISSIVE: u32       = 0b01000;
    pub const DYNAMIC: u32        = 0b10000;
}

//end tile.rs