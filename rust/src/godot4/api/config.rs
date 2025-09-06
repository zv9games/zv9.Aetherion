//C:/ZV9/zv9.aetherion/rust/src/godot4/api/config.rs

use rand::Rng;
use godot::prelude::*;

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

//end config.rs