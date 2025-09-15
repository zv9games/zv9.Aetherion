//C:/ZV9/zv9.aetherion/rust/src/zv9_godot_interface_api_generator.rs

/// ✅ Suggestions for godot4/api/generator.rs

// 🔧 Expand pattern logic:
//     - Replace placeholder `generate_pattern_tile()` with real pattern definitions
//     - Consider using a `PatternRegistry` or external config for extensibility

// 🧩 Add support for tile flags and metadata:
//     - Allow `generate_noise_tile()` and `generate_pattern_tile()` to set flags
//     - Useful for collision, visibility, or gameplay logic

// 🚦 Add input validation:
//     - Clamp or sanitize `x`, `y`, and `seed` values
//     - Emit warnings for unknown pattern names

// 📚 Document expected usage from GDScript:
//     - Clarify how to call `generate_noise()` and `generate_pattern()`
//     - Note what the returned dictionary contains and how to use it

// 🧪 Add unit tests for tile generation:
//     - Validate alternate ID hashing, pattern mapping, and dictionary conversion
//     - Ensure consistency across seeds and coordinates

// 🧼 Optional: Add tile preview or debug output:
//     - `fn describe_tile(tile: &TileInfo) -> String`
//     - Useful for logging or editor overlays

// 🚀 Future: Add support for batch generation:
//     - `fn generate_grid(...) -> Array<Dictionary>`
//     - Enables procedural map creation directly from GDScript

// 🧠 Consider exposing generation presets:
//     - e.g. `fn generate_tile_by_profile(profile: GString, x: i32, y: i32)`
//     - Useful for biome-aware or theme-based generation


use godot::prelude::*;
use crate::zv9_prelude::*;

/// Godot-facing generator node for procedural tile creation.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionGenerator;

#[godot_api]
impl AetherionGenerator {
	#[allow(dead_code)]
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
		let _ = dict.insert("source_id", tile.source_id);
		let _ = dict.insert("atlas_coords", Vector2i::from(tile.atlas_coords));
		let _ = dict.insert("alternate_id", tile.alternate_id);
		let _ = dict.insert("rotation", tile.rotation);
		let _ = dict.insert("layer", tile.layer);
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
		variant_id: None,
		frame_count: None,
		animation_speed: None,
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
		variant_id: None,
		frame_count: None,
		animation_speed: None,
	}

}

// the end