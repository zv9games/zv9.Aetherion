use godot::prelude::*;
use godot_macros::gdextension;

#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 📦 Version info
pub const VERSION: &str = "0.1.0";

/// 📦 Prelude
pub mod zv9_prelude {
    include!("zv9_prelude.rs");
}

/// 🧩 Centralized init hook
pub fn init_all() {
    use aetherion_core::util::logging::log_event;
    use aetherion_core::trailkeeper::entry::EventType;

    log_event!(
        EventType::System,
        "engine",
        format!("Aetherion boot sequence started (v{})", VERSION)
    );
}

/// 📚 Modular includes
#[path = "zv9_lib_interface.rs"]
mod zv9_lib_interface;



#[path = "zv9_godot_interface_emulator.rs"]
mod zv9_godot_interface_emulator;

#[path = "zv9_aetherion_engine_queue.rs"]
mod zv9_aetherion_engine_queue;

/// 🔓 Re-exports for binary access

// Prelude
pub use zv9_prelude::*;

// Core (from aetherion_core)
pub use aetherion_core::core::*;
pub use aetherion_core::core::runtime::start as start_runtime;
pub use aetherion_core::core::conductor::{Conductor, ProcCommand};

// Interface
pub use zv9_lib_interface::*;
pub use aetherion_core::interface::GodotSync;


// Emulator
pub use zv9_godot_interface_emulator::{
    FakeTileMap,
    TileMapInterface,
    test_generation_and_placement_cli,
};

// Queue Inspector
pub use zv9_aetherion_engine_queue::inspect_pending_queue;

// Pipeline
pub use aetherion_core::pipeline::data::MapDataChunk;

pub mod pipeline_builder {
    pub mod bitmask {
        pub use aetherion_core::pipeline::builder::*;
    }
}

/// 🧪 Tests
#[cfg(test)]
mod integration_tests {
    // Add test modules here
}

/// 🚀 Godot Extension Entry
struct AetherionEXT;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEXT {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion is summoned.");
        }
    }
}
