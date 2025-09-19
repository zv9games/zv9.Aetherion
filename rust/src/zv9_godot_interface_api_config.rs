//C:/ZV9/zv9.aetherion/rust/src/zv9_godot_interface_api_config.rs
use rand::Rng;
use godot::prelude::*;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// ⚙️ AetherionConfig — Configuration node for exposing procedural engine settings to Godot.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionConfig {
    /// Size of each tile in pixels.
    #[export]
    pub tile_size: i32,

    /// Width of each chunk in tiles.
    #[export]
    pub chunk_width: i32,

    /// Height of each chunk in tiles.
    #[export]
    pub chunk_height: i32,

    /// Procedural seed used for generation.
    #[export]
    pub seed: i64,

    /// Enables voxel-based rendering mode.
    #[export]
    pub enable_voxel_mode: bool,
}

#[godot_api]
impl AetherionConfig {
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
        log_component!("AetherionConfig", "Configuration node for procedural engine settings"); // ✅ No macro import needed
    }

    /// Returns the total number of tiles in a chunk.
    #[func]
    fn get_chunk_area(&self) -> i32 {
        self.chunk_width * self.chunk_height
    }

    /// Regenerates the procedural seed.
    #[func]
    fn regenerate_seed(&mut self) {
        self.seed = rand::thread_rng().gen_range(0..=i64::MAX);
    }
}



// the end