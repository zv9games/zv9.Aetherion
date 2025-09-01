//! 🌌 AetherionEngine — Procedural core for Godot, powered by Rust
//!
//! This crate provides:
//! - Threaded terrain generation
//! - Signal dispatch system
//! - Runtime orchestration for expansive, dimension-agnostic worlds

extern crate godot_macros;

pub mod godot4;
pub mod aetherion;

use godot::prelude::*;
use godot_macros::gdextension;

use godot4::api::engine::AetherionEngine;
use godot4::signals::AetherionSignals;

/// 🚀 Entry point for the Aetherion GDExtension.
/// Hooks into Godot's lifecycle. Class registration is handled automatically.
struct AetherionExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion GDExtension initialized and ready.");
        }
    }
}
