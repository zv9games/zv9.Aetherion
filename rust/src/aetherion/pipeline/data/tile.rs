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

    /// Optional alternate ID for visual variation or animation.
    pub alternate_id: i32,

    /// Rotation in 90° steps (0–3).
    pub rotation: u8,

    /// Layer index for multi-layer maps.
    pub layer: u8,

    /// Bitmask for collision, visibility, or custom flags.
    pub flags: u32,
}

/// Bitmask flags for tile behavior and rendering.
/// Use bitwise OR to combine multiple flags.
pub mod tile_flags {
    /// Tile blocks movement or physics.
    pub const COLLIDABLE: u32 = 0b0001;

    /// Tile is visible in the current layer.
    pub const VISIBLE: u32 = 0b0010;

    /// Tile can be interacted with (e.g. clicked, triggered).
    pub const INTERACTIVE: u32 = 0b0100;

    /// Tile emits light or affects lighting.
    pub const EMISSIVE: u32 = 0b1000;

    /// Tile is part of a dynamic system (e.g. animated, destructible).
    pub const DYNAMIC: u32 = 0b10000;
}
