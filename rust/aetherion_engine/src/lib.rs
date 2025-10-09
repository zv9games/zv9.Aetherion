use godot::prelude::*;
use godot::init::{ExtensionLibrary, InitLevel};
mod zv9_godot_interface_api_engine;

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
// ─── Godot Interface Modules ───────────────────────────────────────────────────
//

// Engine Core
mod zv9_godot_interface_api_engine_core;
mod zv9_godot_interface_api_engine_signals;
pub mod zv9_godot_interface_api_engine_util;

// API Modules
mod zv9_godot_interface_api_config;
mod zv9_godot_interface_api_generator;
mod zv9_godot_interface_api_map;
mod zv9_godot_interface_api_oracle;
mod zv9_godot_interface_api_signals;

// Messaging & Sync
mod zv9_godot_interface_messaging_messages;
mod zv9_godot_interface_messaging_sync;

// Diagnostics & Controls
mod zv9_godot_interface_interface_controls;
mod zv9_godot_interface_interface_diagnostics;

// Signal Definitions
mod zv9_godot_interface_signals_definitions;
mod zv9_godot_interface_signals_dispatch;

// Bindings & Extensions
mod zv9_godot_interface_bindings_godot_types;
mod zv9_godot_interface_emulator;
mod zv9_godot_interface_map_ext;

// Queue Inspector
mod zv9_aetherion_engine_queue;

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

// API Modules
pub use zv9_godot_interface_api_config::*;
pub use zv9_godot_interface_api_generator::*;
pub use zv9_godot_interface_api_map::*;
pub use zv9_godot_interface_api_oracle::*;
pub use zv9_godot_interface_api_signals::*;

// Messaging & Sync
// pub use zv9_godot_interface_messaging_messages::*;
pub use zv9_godot_interface_messaging_sync::*;

// Diagnostics & Controls
pub use zv9_godot_interface_interface_controls::*;
pub use zv9_godot_interface_interface_diagnostics::*;

// Signal Definitions
pub use zv9_godot_interface_signals_definitions::*;
pub use zv9_godot_interface_signals_dispatch::*;

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
// ─── Godot Extension Entry (Trait-Based) ───────────────────────────────────────
//

pub struct AetherionExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 AetherionExtension → Scene init level reached");
            init_all();
        }
    }
}

