// aetherion_godot/src/lib.rs

// 🛑 Declare sub-modules (no change needed here)
pub mod aetherion_engine;
pub mod aetherion_signals;
pub mod aetherion_oracle;

use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel}; 

// --- GDEXTENSION ENTRY POINT ---

struct AetherionExtension;

// 🛑 FIX: Use the simple, stable, declarative ExtensionLibrary implementation.
// This is the correct signature for your version and relies on the #[derive(GodotClass)] 
// in the sub-modules to perform registration.
#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    fn on_level_init(_level: InitLevel) {
        // Leave the body empty. Registration happens automatically via macros.
    }
}