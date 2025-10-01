// ─── Godot Builtins & UI ───────────────────────────────────────────────────────
pub use godot::builtin::*;
pub use godot::classes::*;

// ─── Core Data Types ───────────────────────────────────────────────────────────
pub use aetherion_core::pipeline::data::{MapDataChunk, TileInfo};

// ─── Shared Math & Types ───────────────────────────────────────────────────────
pub use aetherion_core::zv9_shared_math::{clamp, TAU};
pub use aetherion_core::zv9_shared_types::{Coord, EntityId, SerializableVector2i, Timestamp};

// ─── Shared Traits & Spatial Logic ─────────────────────────────────────────────
pub use aetherion_core::zv9_shared_traits::{Serializable, Tickable};
pub use aetherion_core::zv9_shared_spatial::{all_neighbors, cardinal_neighbors, in_bounds};
pub use aetherion_core::zv9_shared_grid2d::Grid2D;
pub use aetherion_core::zv9_shared_grid_bounds::GridBounds;

// ─── Codegen & DSL ─────────────────────────────────────────────────────────────
pub use aetherion_core::zv9_aetherion_codegen_config::*;
pub use aetherion_core::zv9_aetherion_codegen_dsl::*;
pub use aetherion_core::zv9_aetherion_codegen_emitter::*;
pub use aetherion_core::zv9_aetherion_codegen_parser::*;

// ─── Generator & Patterns ──────────────────────────────────────────────────────
pub use aetherion_core::zv9_aetherion_generator_noise::*;
pub use aetherion_core::zv9_aetherion_generator_noise_config::*;
pub use aetherion_core::zv9_aetherion_generator_pattern_type::*;
pub use aetherion_core::zv9_aetherion_generator_patterns::*;

// ─── Structure Placement & Generation ──────────────────────────────────────────
pub use aetherion_core::zv9_aetherion_structure_generation::{
    ExternalNoiseType, MapBuildOptions, generate_virtual_field, tile_at,
};
pub use aetherion_core::zv9_aetherion_structure_placement::*;

// ─── Pipeline Builders ─────────────────────────────────────────────────────────
pub use aetherion_core::pipeline::builder::{
    ChunkDelivery, ChunkStreamer, SyncBridge, spawn_map_builder,
};

// ─── Core Runtime & Lifecycle ──────────────────────────────────────────────────
pub use aetherion_core::zv9_aetherion_core_conductor::*;
pub use aetherion_core::zv9_aetherion_core_dimension::*;
pub use aetherion_core::zv9_aetherion_core_lifecycle::*;
pub use aetherion_core::zv9_aetherion_core_runtime::*;

// ─── Interaction Modules ───────────────────────────────────────────────────────
pub use aetherion_core::zv9_aetherion_interaction_modifiers::*;
pub use aetherion_core::zv9_aetherion_interaction_tools::*;

// ─── Utility Modules ───────────────────────────────────────────────────────────
pub use aetherion_core::zv9_util_config::*;
pub use aetherion_core::zv9_util_direction::*;
pub use aetherion_core::zv9_util_logging::*;
pub use aetherion_core::zv9_util_position::*;
pub use aetherion_core::zv9_util_profiling::*;
pub use aetherion_core::zv9_util_time::*;
pub use aetherion_core::zv9_util_timer::*;
pub use aetherion_core::zv9_util_velocity::*;

// ─── Trailkeeper System ────────────────────────────────────────────────────────
pub use aetherion_core::zv9_trailkeeper_collector::*;
pub use aetherion_core::zv9_trailkeeper_config::*;
pub use aetherion_core::zv9_trailkeeper_entry::*;
pub use aetherion_core::zv9_trailkeeper_export::*;
pub use aetherion_core::zv9_trailkeeper_macros::*;
pub use aetherion_core::zv9_trailkeeper_registry::*;
pub use aetherion_core::zv9_trailkeeper_scan::*;
pub use aetherion_core::zv9_trailkeeper_watch::*;

// ─── Godot Messaging & Sync ────────────────────────────────────────────────────
pub use crate::zv9_lib_interface::zv9_godot_interface_messaging_messages::*;
pub use crate::zv9_lib_interface::zv9_godot_interface_messaging_sync::*;

// ─── Godot API Modules ─────────────────────────────────────────────────────────
pub use crate::zv9_lib_interface::{
    zv9_godot_interface_api_config::*,
    zv9_godot_interface_api_engine::*,
    zv9_godot_interface_api_generator::*,
    zv9_godot_interface_api_map::*,
    zv9_godot_interface_api_oracle::*,
    zv9_godot_interface_api_signals::*,
};

// ─── Diagnostics & Controls ────────────────────────────────────────────────────
pub use crate::zv9_lib_interface::{
    zv9_godot_interface_interface_controls::*,
    zv9_godot_interface_interface_diagnostics::*,
};

// ─── Signal Definitions & Dispatch ─────────────────────────────────────────────
pub use crate::zv9_lib_interface::{
    zv9_godot_interface_signals_definitions::*,
    zv9_godot_interface_signals_dispatch::*,
};
