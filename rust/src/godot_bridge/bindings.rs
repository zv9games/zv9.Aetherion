use godot::prelude::*;
use crate::godot_bridge::commands::AetherionEngine;
use crate::godot_bridge::signals::AetherionSignals;

/// This function is no longer needed in the current gdext architecture.
/// Class registration is handled automatically via `#[derive(GodotClass)]` and `#[class(init)]` on each struct.
/// If you still want to keep this function for organizational purposes, you can leave it empty or use it for logging.
pub fn register_bindings() {
    godot_print!("Bindings registered — classes are wired via macros.");
}
