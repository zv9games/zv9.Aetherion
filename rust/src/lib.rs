//C:/ZV9/zv9.aetherion/rust/src/lib.rs
use godot::prelude::*;
use godot_macros::gdextension;

pub mod aetherion {
    pub mod codegen;
    pub mod core;
    pub mod generator;
    pub mod interaction;
    pub mod pipeline;
    pub mod structure;
}

pub mod util;
pub mod tests;
pub mod shared;
pub mod examples;
pub mod godot4;
pub mod trailkeeper;





struct AetherionEXT;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEXT {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion is summoned.");
        }
    }
}

//end lib.rs