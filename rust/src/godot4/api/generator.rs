use godot::prelude::*;
use crate::aetherion::pipeline::data::TileInfo;

/// Godot-facing generator node for procedural tile creation.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct AetherionGenerator;

#[godot_api]
impl AetherionGenerator {
    #[func]
    fn _ready(&self) {
        godot_print!("🌱 AetherionGenerator ready.");
    }

    /// Generates a tile using noise at the given coordinates.
    #[func]
    fn generate_noise(&self, x: f32, y: f32, seed: i64) -> Dictionary {
        let tile = generate_noise_tile(x, y, seed);
        Self::tile_to_dict(tile)
    }

    /// Generates a tile using a named pattern.
    #[func]
    fn generate_pattern(&self, pattern_name: String, x: i32, y: i32) -> Dictionary {
        let tile = generate_pattern_tile(&pattern_name, x, y);
        Self::tile_to_dict(tile)
    }

    /// Converts a TileInfo into a Godot Dictionary.
    fn tile_to_dict(tile: TileInfo) -> Dictionary {
        let mut dict = Dictionary::new();
        dict.insert("id", tile.id);
        dict.insert("meta", tile.meta);
        dict.insert("visible", tile.visible);
        dict.insert("layer", tile.layer);
        dict
    }
}

/// Generates a tile using noise at the given coordinates.
pub fn generate_noise_tile(x: f32, y: f32, seed: i64) -> TileInfo {
    TileInfo {
        id: format!("noise_{}_{}", x, y),
        meta: seed.to_string(),
        visible: true,
        layer: 0,
    }
}

/// Generates a tile using a named pattern.
pub fn generate_pattern_tile(pattern_name: &str, x: i32, y: i32) -> TileInfo {
    TileInfo {
        id: format!("pattern_{}_{}_{}", pattern_name, x, y),
        meta: "pattern".into(),
        visible: true,
        layer: 1,
    }
}
