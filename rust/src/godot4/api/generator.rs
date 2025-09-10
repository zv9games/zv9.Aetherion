use godot::prelude::*;
use crate::aetherion::pipeline::data::TileInfo;

/// Godot-facing generator node for procedural tile creation.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionGenerator;

#[godot_api]
impl AetherionGenerator {
    fn init(_base: Base<Node>) -> Self {
        Self
    }

    #[func]
    fn _ready(&self) {
        godot_print!("🌱 AetherionGenerator ready.");
    }

    /// Generates a tile using noise at the given coordinates and seed.
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
        dict.insert("source_id", tile.source_id);
        dict.insert("atlas_coords", Vector2i::from(tile.atlas_coords));
        dict.insert("alternate_id", tile.alternate_id);
        dict.insert("rotation", tile.rotation);
        dict.insert("layer", tile.layer);
        dict
    }
}

/// Generates a tile using noise at the given coordinates and seed.
/// This version uses a simple hash to vary alternate_id based on seed.
pub fn generate_noise_tile(x: f32, y: f32, seed: i64) -> TileInfo {
    let hash = ((x * 73856093.0) as i64 ^ (y * 19349663.0) as i64 ^ seed) & 0xFFFF;
    let alt = (hash % 4) as i32;

    TileInfo {
        source_id: 0,
        atlas_coords: Vector2i::new(x as i32, y as i32).into(),
        alternate_id: alt,
        rotation: 0,
        layer: 0,
        flags: 0,
    }
}

/// Generates a tile using a named pattern.
/// Currently uses a placeholder logic; pattern_name will drive future variants.
pub fn generate_pattern_tile(pattern_name: &str, x: i32, y: i32) -> TileInfo {
    let alt = match pattern_name {
        "floor" => 1,
        "wall" => 2,
        "path" => 3,
        _ => 0,
    };

    TileInfo {
        source_id: 1,
        atlas_coords: Vector2i::new(x, y).into(),
        alternate_id: alt,
        rotation: 0,
        layer: 1,
        flags: 0,
    }
}
