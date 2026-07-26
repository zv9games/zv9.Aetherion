// aetherion_godot/src/lib.rs

//! The FFI layer for zv9.Aetherion, providing GDExtension bindings
//! for Godot to interface with the Rust core.

// -------------------------------------------------------------------------------------------------
// MODULE DECLARATIONS
// -------------------------------------------------------------------------------------------------

// Declare sub-modules containing the Godot-facing classes.
pub mod aetherion_engine;
pub mod aetherion_signals;
pub mod aetherion_oracle;

// -------------------------------------------------------------------------------------------------
// GODOT BINDING SETUP
// -------------------------------------------------------------------------------------------------

use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel}; // FIX: Removed trailing No-Break Space

/// The primary struct for the Aetherion GDExtension library.
/// This acts as the entry point for Godot's C++ binding layer.
struct AetherionExtension;

/// The GDExtension entry point, responsible for loading and unloading the library.
/// Registration of classes (`AetherionEngine`, etc.) is handled automatically
/// by the `#[derive(GodotClass)]` macro in the respective sub-modules.
#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    /// Called when the library is initialized at a specific level (Editor/Scene/Game).
    fn on_level_init(_level: InitLevel) {
        // Leave empty. Class registration is declarative via the macros.
    }
}