use godot::prelude::*;
use crate::aetherion::generator::{generate_noise_tile, generate_pattern_tile};
use crate::aetherion::pipeline::data::TileInfo;

/// AetherionGenerator — exposes procedural generation to Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AetherionGenerator;

#[godot_api]
impl AetherionGenerator {
    #[func]
    fn _ready(&self) {
        godot_print!("🌱 AetherionGenerator ready.");
    }

    /// Generate a tile using noise at given coordinates
    #[func]
    fn generate_noise(&self, x: f32, y: f32, seed: i64) -> Dictionary {
        let tile = generate_noise_tile(x, y, seed);
        Self::tile_to_dict(tile)
    }

    /// Generate a tile using a named pattern
    #[func]
    fn generate_pattern(&self, pattern_name: String, x: i32, y: i32) -> Dictionary {
        let tile = generate_pattern_tile(&pattern_name, x, y);
        Self::tile_to_dict(tile)
    }

    /// Convert TileInfo to Godot Dictionary
    fn tile_to_dict(tile: TileInfo) -> Dictionary {
        let mut dict = Dictionary::new();
        dict.insert("id", tile.id);
        dict.insert("meta", tile.meta);
        dict.insert("visible", tile.visible);
        dict.insert("layer", tile.layer);
        dict
    }
}
