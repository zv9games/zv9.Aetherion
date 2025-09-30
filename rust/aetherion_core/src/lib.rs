// ─── Prelude & Delivery ────────────────────────────────────────────────────────

pub mod zv9_prelude;
pub mod zv9_aetherion_pipeline_builder_dummy_delivery;
pub use zv9_aetherion_pipeline_builder_dummy_delivery::DummyDelivery;

// ─── Codegen ───────────────────────────────────────────────────────────────────

mod zv9_aetherion_codegen_config;
mod zv9_aetherion_codegen_dsl;
mod zv9_aetherion_codegen_emitter;
mod zv9_aetherion_codegen_parser;

pub mod codegen {
    pub mod config {
        pub use crate::zv9_aetherion_codegen_config::*;
    }
    pub mod dsl {
        pub use crate::zv9_aetherion_codegen_dsl::*;
    }
    pub mod emitter {
        pub use crate::zv9_aetherion_codegen_emitter::*;
    }
    pub mod parser {
        pub use crate::zv9_aetherion_codegen_parser::*;
    }
}

// ─── Core Engine ───────────────────────────────────────────────────────────────

mod zv9_aetherion_core_conductor;
mod zv9_aetherion_core_dimension;
mod zv9_aetherion_core_lifecycle;
mod zv9_aetherion_core_runtime;

pub mod core {
    pub mod conductor {
        pub use crate::zv9_aetherion_core_conductor::*;
    }
    pub mod dimension {
        pub use crate::zv9_aetherion_core_dimension::*;
    }
    pub mod lifecycle {
        pub use crate::zv9_aetherion_core_lifecycle::*;
    }
    pub mod runtime {
        pub use crate::zv9_aetherion_core_runtime::*;
    }
}

// ─── Generator ─────────────────────────────────────────────────────────────────

mod zv9_aetherion_generator_noise;
mod zv9_aetherion_generator_noise_config;
mod zv9_aetherion_generator_patterns;
mod zv9_aetherion_generator_pattern_type;

pub mod generator {
    pub mod noise {
        pub use crate::zv9_aetherion_generator_noise::*;
    }
    pub mod noise_config {
        pub use crate::zv9_aetherion_generator_noise_config::*;
    }
    pub mod patterns {
        pub use crate::zv9_aetherion_generator_patterns::*;
    }
    pub mod pattern_type {
        pub use crate::zv9_aetherion_generator_pattern_type::*;
    }
}

// ─── Interaction ──────────────────────────────────────────────────────────────

mod zv9_aetherion_interaction_modifiers;
mod zv9_aetherion_interaction_tools;

pub mod interaction {
    pub mod modifiers {
        pub use crate::zv9_aetherion_interaction_modifiers::*;
    }
    pub mod tools {
        pub use crate::zv9_aetherion_interaction_tools::*;
    }
}

// ─── Pipeline ──────────────────────────────────────────────────────────────────

mod zv9_aetherion_pipeline_builder_bitmask;
mod zv9_aetherion_pipeline_builder_builder;
mod zv9_aetherion_pipeline_builder_streamer;
mod zv9_aetherion_pipeline_builder_threaded;
mod zv9_aetherion_pipeline_data_chunk;
mod zv9_aetherion_pipeline_data_data;
mod zv9_aetherion_pipeline_data_grid;
mod zv9_aetherion_pipeline_data_tile;

pub mod pipeline {
    pub mod builder {
		pub use crate::zv9_aetherion_pipeline_builder_bitmask::*;
		pub use crate::zv9_aetherion_pipeline_builder_builder::*;
		pub use crate::zv9_aetherion_pipeline_builder_streamer::*;
		pub use crate::zv9_aetherion_pipeline_builder_streamer::SyncBridge;
		pub use crate::zv9_aetherion_pipeline_builder_streamer::ChunkDelivery; // ✅ Added
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

mod zv9_aetherion_structure_generation;
mod zv9_aetherion_structure_placement;

pub mod structure {
    pub use crate::zv9_aetherion_structure_generation::*;
    pub use crate::zv9_aetherion_structure_placement::*;
}

// ─── Shared ────────────────────────────────────────────────────────────────────

mod zv9_shared_messages;
mod zv9_shared_types;
mod zv9_shared_traits;
mod zv9_shared_math;
mod zv9_shared_grid2d;
mod zv9_shared_grid_bounds;
mod zv9_shared_spatial;

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

mod zv9_trailkeeper_collector;
mod zv9_trailkeeper_config;
mod zv9_trailkeeper_entry;
mod zv9_trailkeeper_export;
#[macro_use] // Runtime macros: adding, subtracting, multiplying, dividing
mod zv9_trailkeeper_macros;
mod zv9_trailkeeper_registry;
mod zv9_trailkeeper_scan;
mod zv9_trailkeeper_watch;

pub mod trailkeeper {
    pub mod collector {
        pub use crate::zv9_trailkeeper_collector::*;
    }
    pub mod config {
        pub use crate::zv9_trailkeeper_config::*;
    }
    pub mod entry {
        pub use crate::zv9_trailkeeper_entry::*;
    }
    pub mod export {
        pub use crate::zv9_trailkeeper_export::*;
    }
    pub mod macros {
        pub use crate::zv9_trailkeeper_macros::*;
    }
    pub mod registry {
        pub use crate::zv9_trailkeeper_registry::*;
    }
    pub mod scan {
        pub use crate::zv9_trailkeeper_scan::*;
    }
    pub mod watch {
        pub use crate::zv9_trailkeeper_watch::*;
    }
}



// ─── General Utilities ─────────────────────────────────────────────────────────

mod zv9_util_config;
mod zv9_util_direction;
mod zv9_util_logging;
mod zv9_util_position;
mod zv9_util_profiling;
mod zv9_util_random;
mod zv9_util_time;
mod zv9_util_timer;
mod zv9_util_velocity;

pub mod util {
    pub mod config {
        pub use crate::zv9_util_config::*;
    }
    pub mod direction {
        pub use crate::zv9_util_direction::*;
    }
    pub mod logging {
        pub use crate::zv9_util_logging::*;
    }
    pub mod position {
        pub use crate::zv9_util_position::*;
    }
    pub mod profiling {
        pub use crate::zv9_util_profiling::*;
    }
    pub mod random {
        pub use crate::zv9_util_random::*;
    }
    pub mod time {
        pub use crate::zv9_util_time::*;
    }
    pub mod timer {
        pub use crate::zv9_util_timer::*;
    }
    pub mod velocity {
        pub use crate::zv9_util_velocity::*;
    }
}
