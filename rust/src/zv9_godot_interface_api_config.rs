//C:/ZV9/zv9.aetherion/rust/src/zv9_godot_interface_api_config.rs

/// ✅ Suggestions for godot4/api/config.rs

// 🔧 Add validation and clamping:
//     - Ensure `tile_size`, `chunk_width`, and `chunk_height` are positive
//     - Prevent zero or negative values that could break generation

// 🧩 Add support for saving/loading config:
//     - `fn to_dict() -> Dictionary` and `fn from_dict(dict: Dictionary)`
//     - Enables persistence, presets, or editor integration

// 🚦 Add voxel mode configuration:
//     - If `enable_voxel_mode` is true, expose additional fields like `voxel_resolution`, `depth_layers`
//     - Useful for hybrid 2D/3D terrain systems

// 📚 Document intended usage in Godot:
//     - Clarify how this node feeds into the engine pipeline
//     - Note whether it's meant for runtime tweaking or static setup

// 🧪 Add unit tests or GDScript integration tests:
//     - Validate seed regeneration, area calculation, and export behavior

// 🧼 Optional: Add debug summary or display method:
//     - `fn describe_config() -> String`
//     - Useful for logging or editor overlays

// 🚀 Future: Add support for config presets:
//     - e.g. “small map”, “large map”, “voxel test”
//     - Could expose `fn apply_preset(name: GString)`

// 🧠 Consider exposing config as a resource:
//     - Convert to `AetherionConfigResource` for use in scenes or inspector
//     - Improves modularity and reuse across projects


use rand::Rng;
use godot::prelude::*;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// Configuration node for exposing procedural engine settings to Godot.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionConfig {
    #[export]
    pub tile_size: i32,

    #[export]
    pub chunk_width: i32,

    #[export]
    pub chunk_height: i32,

    #[export]
    pub seed: i64,

    #[export]
    pub enable_voxel_mode: bool,
}

#[godot_api]
impl AetherionConfig {
	#[allow(dead_code)]
    fn init(_base: Base<Node>) -> Self {
        Self {
            tile_size: 16,
            chunk_width: 8,
            chunk_height: 8,
            seed: rand::thread_rng().gen_range(0..=i64::MAX),
            enable_voxel_mode: false,
        }
    }

    #[func]
    fn _ready(&self) {
        godot_print!("⚙️ AetherionConfig loaded.");
    }

    #[func]
    fn get_chunk_area(&self) -> i32 {
        self.chunk_width * self.chunk_height
    }

    #[func]
    fn regenerate_seed(&mut self) {
        self.seed = rand::thread_rng().gen_range(0..=i64::MAX);
    }
}

// the end