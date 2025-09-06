//C:/ZV9/zv9.aetherion/rust/src/lib.rs
use godot::prelude::*;
use godot_macros::gdextension;

pub mod aetherion;
pub mod util;

mod godot4;
mod tests;
mod shared;
mod examples;




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