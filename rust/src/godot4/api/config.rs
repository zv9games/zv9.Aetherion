use godot::prelude::*;

/// Configuration node for exposing procedural engine settings to Godot.
#[derive(GodotClass)]
#[class(base = Node)]
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
        self.seed = godot4::api::engine::random::rand_range(0, i64::MAX);
    }
}
