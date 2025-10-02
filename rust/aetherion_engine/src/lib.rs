use godot::prelude::*;
use godot_macros::gdextension;

#[allow(unused_imports)]
use crate::zv9_prelude::*;

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
}

//
// ─── Modular Includes ──────────────────────────────────────────────────────────
//

#[path = "zv9_aetherion_engine_queue.rs"]
mod zv9_aetherion_engine_queue;

#[path = "zv9_godot_interface_emulator.rs"]
mod zv9_godot_interface_emulator;

#[path = "zv9_godot_interface_map_ext.rs"]
mod zv9_godot_interface_map_ext;

#[path = "zv9_lib_interface.rs"]
mod zv9_lib_interface;

//
// ─── Re-exports for Binary Access ──────────────────────────────────────────────
//

// Prelude
pub use zv9_prelude::*;

// Core
pub use aetherion_core::core::*;
#[allow(unused_imports)]
pub use aetherion_core::zv9_aetherion_core_conductor::{Conductor, ProcCommand};
#[allow(unused_imports)]
use aetherion_core::zv9_aetherion_core_runtime::start as start_runtime;

// Interface
pub use zv9_lib_interface::*;
pub use zv9_godot_interface_map_ext::MapDataChunkExt;

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

struct AetherionEXT;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEXT {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion is summoned.");
        }
    }
}
