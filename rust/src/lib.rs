//C:/ZV9/zv9.aetherion/rust/src/lib.rs

/// ✅ Suggestions for lib.rs

// 🔧 Add versioning or build metadata:
//     - e.g. `pub const VERSION: &str = "0.1.0"`
//     - Useful for diagnostics, compatibility checks, and logging

// 🧩 Add centralized init hook:
//     - `fn init_all()` to initialize subsystems (e.g. logging, config, profiling)
//     - Ensures consistent startup behavior across platforms

// 🚦 Improve safety around `gdextension`:
//     - Document why `unsafe` is required and what guarantees are assumed
//     - Consider wrapping risky logic in safe abstractions

// 📚 Document module layout:
//     - Clarify purpose of each top-level module (e.g. `shared`, `godot4`, `trailkeeper`)
–     - Helps contributors and tooling navigate the codebase

// 🧪 Add integration test entry point:
//     - e.g. `#[cfg(test)] mod integration_tests`
//     - Useful for validating Godot bindings and engine behavior

// 🧼 Optional: Add feature flags:
//     - e.g. `#[cfg(feature = "trailkeeper")]` for conditional compilation
//     - Enables modular builds and targeted testing

// 🚀 Future: Add CLI or editor entry point:
//     - e.g. `fn run_editor()` or `fn run_cli()`
//     - Enables standalone tooling or headless workflows

// 🧠 Consider exposing metadata to GDScript:
//     - e.g. `fn get_engine_info() -> Dictionary`
//     - Useful for runtime introspection or editor overlays


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