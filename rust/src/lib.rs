use godot::prelude::*;
use godot_macros::gdextension;


mod core;
mod data_processing;
mod game_logic;
mod logging;
mod prelude;
mod utilities;
mod godot_bridge;

// This struct is your extension entry point.
// It doesn't need any fields—just exists to implement the trait.
struct AetherionExtension;

// This macro wires your extension into Godot's lifecycle.
#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion GDExtension initialized.");
            // No need to manually register classes—macros handle it.
        }
    }
}


