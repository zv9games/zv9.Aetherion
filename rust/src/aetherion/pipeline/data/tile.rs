
/// ✅ Suggestions for aetherion/pipeline/data/tile.rs

// 🔧 Add helper methods for flag manipulation:
//     - `fn has_flag(&self, flag: u32) -> bool`
//     - `fn set_flag(&mut self, flag: u32)`
//     - `fn clear_flag(&mut self, flag: u32)`
//     - Improves ergonomics for gameplay and editor logic

// 🧩 Add support for tile variants or animations:
//     - e.g. `variant_id`, `frame_count`, `animation_speed`
//     - Useful for animated tiles or visual diversity

// 🚦 Add validation or clamping:
//     - Ensure `rotation` is within 0–3
//     - Optionally validate `layer` or `alternate_id` ranges

// 📚 Document flag usage and conventions:
//     - Clarify how each bitmask flag affects rendering or interaction
//     - Could include examples or usage notes

// 🧪 Add unit tests for `to_dictionary()` and flag logic:
//     - Validate dictionary contents and flag combinations

// 🧼 Optional: Add display or debug summary:
//     - `fn describe(&self) -> String`
//     - Useful for logging, diagnostics, or editor overlays

// 🚀 Future: Add serialization to/from GDScript objects:
//     - e.g. `fn from_dictionary(dict: &Dictionary) -> TileInfo`
//     - Enables round-trip editing and runtime mutation

// 🧠 Consider exposing tile identity or type:
//     - e.g. `fn tile_type(&self) -> TileType`
//     - Could be inferred from `source_id` or `flags`


use serde::{Deserialize, Serialize};
use godot::prelude::*;
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

impl TileInfo {
    /// Converts tile metadata into a Godot Dictionary.
    pub fn to_dictionary(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        let _ = dict.insert("source_id", self.source_id);
        let _ = dict.insert("alternate_id", self.alternate_id);
        let _ = dict.insert("rotation", self.rotation);
        let _ = dict.insert("layer", self.layer);
        let _ = dict.insert("flags", self.flags);
        let _ = dict.insert("atlas_x", self.atlas_coords.x);
        let _ = dict.insert("atlas_y", self.atlas_coords.y);
        dict
    }
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
