// ─── Core Engine Modules ───────────────────────────────────────────────────────
pub mod zv9_aetherion_core_conductor             { include!("zv9_aetherion_core_conductor.rs"); }
pub mod zv9_aetherion_core_dimension             { include!("zv9_aetherion_core_dimension.rs"); }
pub mod zv9_aetherion_core_lifecycle             { include!("zv9_aetherion_core_lifecycle.rs"); }
pub mod zv9_aetherion_core_runtime               { include!("zv9_aetherion_core_runtime.rs"); }

// ─── Pipeline Modules ──────────────────────────────────────────────────────────
pub mod zv9_aetherion_pipeline_builder_bitmask   { include!("zv9_aetherion_pipeline_builder_bitmask.rs"); }
pub mod zv9_aetherion_pipeline_data_chunk        { include!("zv9_aetherion_pipeline_data_chunk.rs"); }
pub mod zv9_aetherion_pipeline_data_grid         { include!("zv9_aetherion_pipeline_data_grid.rs"); }
pub mod zv9_aetherion_pipeline_data_tile         { include!("zv9_aetherion_pipeline_data_tile.rs"); }
pub mod zv9_aetherion_pipeline_data_vector       { include!("zv9_aetherion_pipeline_data_vector.rs"); }

// ─── Core Re-exports ───────────────────────────────────────────────────────────
pub mod core {
    pub mod runtime {
        pub use super::super::zv9_aetherion_core_runtime::start;
    }

    pub use super::zv9_aetherion_core_conductor::*;
    pub use super::zv9_aetherion_core_dimension::*;
    pub use super::zv9_aetherion_core_lifecycle::*;
}

// ─── Core Types ────────────────────────────────────────────────────────────────
pub use zv9_aetherion_core_conductor::{Conductor, ProcCommand};
pub use zv9_aetherion_core_runtime::start;

pub use zv9_aetherion_pipeline_builder_bitmask::*;
//pub use zv9_aetherion_pipeline_data_chunk::MapDataChunk;
pub use zv9_aetherion_pipeline_data_grid::*;
pub use zv9_aetherion_pipeline_data_tile::*;
pub use zv9_aetherion_pipeline_data_vector::*;

// ─── Shared Utilities ──────────────────────────────────────────────────────────
pub mod zv9_shared_grid2d                     { include!("zv9_shared_grid2d.rs"); }
pub mod zv9_shared_grid_bounds                { include!("zv9_shared_grid_bounds.rs"); }
pub mod zv9_shared_math                       { include!("zv9_shared_math.rs"); }
pub mod zv9_shared_spatial                    { include!("zv9_shared_spatial.rs"); }
pub mod zv9_shared_traits                     { include!("zv9_shared_traits.rs"); }
pub mod zv9_shared_types                      { include!("zv9_shared_types.rs"); }

// ─── Codegen & DSL ─────────────────────────────────────────────────────────────
pub mod zv9_aetherion_codegen_config          { include!("zv9_aetherion_codegen_config.rs"); }
pub mod zv9_aetherion_codegen_dsl             { include!("zv9_aetherion_codegen_dsl.rs"); }
pub mod zv9_aetherion_codegen_emitter         { include!("zv9_aetherion_codegen_emitter.rs"); }
pub mod zv9_aetherion_codegen_parser          { include!("zv9_aetherion_codegen_parser.rs"); }

// ─── Generator & Patterns ──────────────────────────────────────────────────────
pub mod zv9_aetherion_generator_noise         { include!("zv9_aetherion_generator_noise.rs"); }
pub mod zv9_aetherion_generator_noise_config  { include!("zv9_aetherion_generator_noise_config.rs"); }
pub mod zv9_aetherion_generator_pattern_type  { include!("zv9_aetherion_generator_pattern_type.rs"); }
pub mod zv9_aetherion_generator_patterns      { include!("zv9_aetherion_generator_patterns.rs"); }

// ─── Pipeline Builders ─────────────────────────────────────────────────────────
pub mod zv9_aetherion_pipeline_builder_builder   { include!("zv9_aetherion_pipeline_builder_builder.rs"); }
pub mod zv9_aetherion_pipeline_builder_streamer  { include!("zv9_aetherion_pipeline_builder_streamer.rs"); }
pub mod zv9_aetherion_pipeline_builder_threaded  { include!("zv9_aetherion_pipeline_builder_threaded.rs"); }

// ─── Interaction ───────────────────────────────────────────────────────────────
pub mod zv9_aetherion_interaction_modifiers   { include!("zv9_aetherion_interaction_modifiers.rs"); }
pub mod zv9_aetherion_interaction_tools       { include!("zv9_aetherion_interaction_tools.rs"); }

// ─── Structure Placement ───────────────────────────────────────────────────────
pub mod zv9_aetherion_structure_generation    { include!("zv9_aetherion_structure_generation.rs"); }
pub mod zv9_aetherion_structure_placement     { include!("zv9_aetherion_structure_placement.rs"); }
