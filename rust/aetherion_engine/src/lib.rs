use godot::prelude::*;

/// 📦 Version info
pub const VERSION: &str = "0.1.0";

//
// ─── Prelude ───────────────────────────────────────────────────────────────────
//

pub mod zv9_prelude {
    include!("zv9_prelude.rs");
}

//
// ─── Sync Bridge ───────────────────────────────────────────────────────────────
//

pub mod zv9_aetherion_sync_bridge;

//
// ─── Centralized Init Hook ─────────────────────────────────────────────────────
//

pub fn init_all() {
    use aetherion_core::log_event;
    use aetherion_core::zv9_trailkeeper_entry::EventType;

    log_event!(
        EventType::System,
        "engine",
        format!("Aetherion boot sequence started (v{})", VERSION)
    );

    godot_print!("🧭 init_all() → Boot sequence logged.");
}

//
// ─── Modular Includes ──────────────────────────────────────────────────────────
//

// Engine Modules
mod zv9_godot_interface_api_engine_core;
mod zv9_godot_interface_api_engine_signals;
mod zv9_godot_interface_api_engine_util; // optional utilities

// Other Interface Modules
mod zv9_aetherion_engine_queue;
mod zv9_godot_interface_emulator;
mod zv9_godot_interface_map_ext;
mod zv9_lib_interface;

//
// ─── Re-exports for Binary Access ──────────────────────────────────────────────
//

// Prelude
pub use zv9_prelude::*;

// Core
pub use aetherion_core::core::*;
pub use aetherion_core::zv9_aetherion_core_conductor::{Conductor, ProcCommand};

// Engine Interface
pub use zv9_godot_interface_api_engine_core::*;
pub use zv9_godot_interface_api_engine_signals::*;
pub use zv9_godot_interface_api_engine_util::*;

// Emulator
pub use zv9_godot_interface_emulator::{
    FakeTileMap,
    TileMapInterface,
    test_generation_and_placement_cli,
};

// Queue Inspector
pub use zv9_aetherion_engine_queue::inspect_pending_queue;

// Map Extensions
pub use zv9_godot_interface_map_ext::MapDataChunkExt;

// Pipeline
pub use aetherion_core::pipeline::data::MapDataChunk;

// Pipeline Builder
pub mod pipeline_builder {
    pub mod bitmask {
        pub use aetherion_core::pipeline::builder::{
            ChunkStreamer,
            ChunkDelivery,
            SyncBridge,
        };
    }
}

//
// ─── Tests ─────────────────────────────────────────────────────────────────────
//

#[cfg(test)]
mod integration_tests {
    // Add test modules here
}

//
// ─── Godot Extension Entry ─────────────────────────────────────────────────────
//

#[derive(Default)]
struct AetherionEXT;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEXT {
    fn on_level_init(_level: InitLevel) {
        godot_print!("🚀 Aetherion is summoned.");
        init_all();
    }
}
