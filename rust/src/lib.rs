use godot::prelude::*;

mod core;
mod data_processing;
mod godot_bridge;
mod game_logic;
mod logging;
mod utilities;
mod prelude;

struct AetherionEngine;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEngine {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("Aetherion GDExtension initialized!");
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("Aetherion GDExtension shut down!");
        }
    }
}
