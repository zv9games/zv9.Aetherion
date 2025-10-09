use godot::prelude::*;
#[allow(unused_imports)]
use crate::zv9_prelude::*;
use aetherion_core::log_component;

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
    

    #[func]
    fn _ready(&self) {
        godot_print!("⚙️ AetherionConfig loaded.");
        log_component!("AetherionConfig", "Configuration node for procedural engine settings");
    }

    /// Returns the total number of tiles in a chunk.
    #[func]
    fn get_chunk_area(&self) -> i32 {
        let area = self.chunk_width * self.chunk_height;
        godot_print!("📐 Chunk area: {} tiles ({}×{})", area, self.chunk_width, self.chunk_height);
        area
    }

    /// Regenerates the procedural seed.
    #[func]
    fn regenerate_seed(&mut self) {
        self.seed = rand::random_range(0..=i64::MAX);
        godot_print!("🌱 Seed regenerated → {}", self.seed);
    }
}
