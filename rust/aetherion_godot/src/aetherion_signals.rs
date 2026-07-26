// aetherion_godot/src/aetherion_signals.rs

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
// Cleaned Imports: Dictionary is the only Godot type needed besides core types.
use godot::builtin::Dictionary; 

// -------------------------------------------------------------------------------------------------
// AETHERION SIGNALS GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// A dedicated Godot Node class used purely for emitting signals from the Rust core back to GDScript.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct AetherionSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl AetherionSignals {
    pub fn init(base: Base<Node>) -> Self {
        AetherionSignals {
            base,
        }
    }
    
    // --- Signal Declarations ---

    #[signal]
    fn build_map_start();

    /// Emits the full chunk data as a dictionary for asynchronous loading.
    #[signal]
    fn chunk_data_ready(chunk_data_dictionary: godot::builtin::Dictionary);
    
    #[signal]
    fn build_map_complete();

    // --- Signal Emitter Functions ---

    /// Emits the signal that the map build process has started.
    #[func]
    pub fn emit_build_map_start(&mut self) {
        self.base_mut().emit_signal("build_map_start", &[]);
    }

    /// Emits the full chunk data dictionary for asynchronous loading.
    #[func]
    pub fn emit_chunk_data_ready(&mut self, chunk_data_dictionary: godot::builtin::Dictionary) {
        // The Dictionary is converted to a Variant before being passed in the signal args array.
        self.base_mut().emit_signal("chunk_data_ready", &[chunk_data_dictionary.to_variant()]);
    }

    /// Emits the signal that the entire map build process is complete.
    #[func]
    pub fn emit_build_map_complete(&mut self) {
        self.base_mut().emit_signal("build_map_complete", &[]);
    }
}