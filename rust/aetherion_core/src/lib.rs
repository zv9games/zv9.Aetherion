// ─── Prelude & Delivery ────────────────────────────────────────────────────────
pub mod zv9_prelude;
pub mod zv9_aetherion_pipeline_builder_dummy_delivery;
pub use zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;

// ─── Codegen ───────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_codegen_config;
pub mod zv9_aetherion_codegen_dsl;
pub mod zv9_aetherion_codegen_emitter;
pub mod zv9_aetherion_codegen_parser;

pub mod codegen {
    pub use crate::zv9_aetherion_codegen_config::*;
    pub use crate::zv9_aetherion_codegen_dsl::*;
    pub use crate::zv9_aetherion_codegen_emitter::*;
    pub use crate::zv9_aetherion_codegen_parser::*;
}

// ─── Core Engine ───────────────────────────────────────────────────────────────
pub mod zv9_aetherion_core_conductor;
pub mod zv9_aetherion_core_dimension;
pub mod zv9_aetherion_core_lifecycle;
pub mod zv9_aetherion_core_runtime;

pub mod core {
    pub use crate::zv9_aetherion_core_conductor::*;
    pub use crate::zv9_aetherion_core_dimension::*;
    pub use crate::zv9_aetherion_core_lifecycle::*;
    pub use crate::zv9_aetherion_core_runtime::*;
}

// ─── Generator ─────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_generator_noise;
pub mod zv9_aetherion_generator_noise_config;
pub mod zv9_aetherion_generator_patterns;
pub mod zv9_aetherion_generator_pattern_type;

pub mod generator {
    pub use crate::zv9_aetherion_generator_noise::*;
    pub use crate::zv9_aetherion_generator_noise_config::*;
    pub use crate::zv9_aetherion_generator_patterns::*;
    pub use crate::zv9_aetherion_generator_pattern_type::*;
}

// ─── Interaction ──────────────────────────────────────────────────────────────
pub mod zv9_aetherion_interaction_modifiers;
pub mod zv9_aetherion_interaction_tools;

pub mod interaction {
    pub use crate::zv9_aetherion_interaction_modifiers::*;
    pub use crate::zv9_aetherion_interaction_tools::*;
}

// ─── Pipeline ──────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_pipeline_builder_bitmask;
pub mod zv9_aetherion_pipeline_builder_builder;
pub mod zv9_aetherion_pipeline_builder_streamer;
pub mod zv9_aetherion_pipeline_builder_threaded;
pub mod zv9_aetherion_pipeline_data_chunk;
pub mod zv9_aetherion_pipeline_data_data;
pub mod zv9_aetherion_pipeline_data_grid;
pub mod zv9_aetherion_pipeline_data_tile;

pub mod pipeline {
    pub mod builder {
        pub use crate::zv9_aetherion_pipeline_builder_bitmask::*;
        pub use crate::zv9_aetherion_pipeline_builder_builder::*;
        pub use crate::zv9_aetherion_pipeline_builder_streamer::*;
        pub use crate::zv9_aetherion_pipeline_builder_streamer::{SyncBridge, ChunkDelivery};
        pub use crate::zv9_aetherion_pipeline_builder_threaded::*;
    }

    pub mod data {
        pub use crate::zv9_aetherion_pipeline_data_chunk::*;
        pub use crate::zv9_aetherion_pipeline_data_data::*;
        pub use crate::zv9_aetherion_pipeline_data_grid::*;
        pub use crate::zv9_aetherion_pipeline_data_tile::*;
    }
}

// ─── Structure ─────────────────────────────────────────────────────────────────
pub mod zv9_aetherion_structure_generation;
pub mod zv9_aetherion_structure_placement;

pub mod structure {
    pub use crate::zv9_aetherion_structure_generation::*;
    pub use crate::zv9_aetherion_structure_placement::*;
}

// ─── Shared ────────────────────────────────────────────────────────────────────
pub mod zv9_shared_messages;
pub mod zv9_shared_types;
pub mod zv9_shared_traits;
pub mod zv9_shared_math;
pub mod zv9_shared_grid2d;
pub mod zv9_shared_grid_bounds;
pub mod zv9_shared_spatial;

pub mod shared {
    pub use crate::zv9_shared_messages::*;
    pub use crate::zv9_shared_types::*;
    pub use crate::zv9_shared_traits::*;
    pub use crate::zv9_shared_math::*;
    pub use crate::zv9_shared_grid2d::*;
    pub use crate::zv9_shared_grid_bounds::*;
    pub use crate::zv9_shared_spatial::*;
}

// ─── Trailkeeper ───────────────────────────────────────────────────────────────
pub mod zv9_trailkeeper_collector;
pub mod zv9_trailkeeper_config;
pub mod zv9_trailkeeper_entry;
pub mod zv9_trailkeeper_export;
#[macro_use]
pub mod zv9_trailkeeper_macros;
pub mod zv9_trailkeeper_registry;
pub mod zv9_trailkeeper_scan;
pub mod zv9_trailkeeper_watch;

pub mod trailkeeper {
    pub use crate::zv9_trailkeeper_collector::*;
    pub use crate::zv9_trailkeeper_config::*;
    pub use crate::zv9_trailkeeper_entry::*;
    pub use crate::zv9_trailkeeper_export::*;
    #[allow(unused_imports)]
	pub use crate::zv9_trailkeeper_macros::*;
    pub use crate::zv9_trailkeeper_registry::*;
    pub use crate::zv9_trailkeeper_scan::*;
    pub use crate::zv9_trailkeeper_watch::*;
}

// ─── General Utilities ─────────────────────────────────────────────────────────
pub mod zv9_util_config;
pub mod zv9_util_direction;
pub mod zv9_util_logging;
pub mod zv9_util_position;
pub mod zv9_util_profiling;
pub mod zv9_util_random;
pub mod zv9_util_time;
pub mod zv9_util_timer;
pub mod zv9_util_velocity;

pub mod util {
    pub use crate::zv9_util_config::*;
    pub use crate::zv9_util_direction::*;
    pub use crate::zv9_util_logging::*;
    pub use crate::zv9_util_position::*;
    pub use crate::zv9_util_profiling::*;
    pub use crate::zv9_util_random::*;
    pub use crate::zv9_util_time::*;
    pub use crate::zv9_util_timer::*;
    pub use crate::zv9_util_velocity::*;
}
