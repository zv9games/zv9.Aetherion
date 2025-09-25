//! Aetherion Core — Pure Rust logic for engine-independent systems

// 🔧 Core utilities
pub mod zv9_util_logging;
pub mod zv9_util_config;
pub mod zv9_util_position;
pub mod zv9_util_direction;
pub mod zv9_util_random;
pub mod zv9_util_time;
pub mod zv9_util_timer;
pub mod zv9_util_velocity;
pub mod zv9_util_profiling;

// 🧪 CLI test harness and tools
pub mod zv9_util_binary_func;
pub mod zv9_util_binary_func2;
pub mod zv9_util_binary_func3;
pub mod zv9_util_binary_menu;

// 🧱 Pipeline and generation
pub mod zv9_aetherion_pipeline_data_tile;
pub mod zv9_aetherion_pipeline_data_vector;
pub mod zv9_aetherion_pipeline_data_grid;
pub mod zv9_aetherion_pipeline_data_chunk;
pub mod zv9_aetherion_pipeline_data_data;
pub mod zv9_aetherion_pipeline_data_build_options;

pub mod zv9_aetherion_generator_noise;
pub mod zv9_aetherion_generator_noise_config;
pub mod zv9_aetherion_generator_patterns;
pub mod zv9_aetherion_generator_pattern_type;

pub mod zv9_aetherion_pipeline_builder_bitmask;
pub mod zv9_aetherion_pipeline_builder_builder;
pub mod zv9_aetherion_pipeline_builder_streamer;
pub mod zv9_aetherion_pipeline_builder_threaded;

// 🧠 Core engine logic
pub mod zv9_aetherion_core_conductor;
pub mod zv9_aetherion_core_dimension;
pub mod zv9_aetherion_core_lifecycle;
pub mod zv9_aetherion_core_runtime;

// 🛠 Interaction and structure
pub mod zv9_aetherion_interaction_modifiers;
pub mod zv9_aetherion_interaction_tools;
pub mod zv9_aetherion_structure_generation;
pub mod zv9_aetherion_structure_placement;

// 📦 Shared types and traits
pub mod zv9_shared_grid_bounds;
pub mod zv9_shared_grid2d;
pub mod zv9_shared_math;
pub mod zv9_shared_spatial;
pub mod zv9_shared_traits;
pub mod zv9_shared_types;

// 🧭 Trailkeeper (if Godot-free)
pub mod zv9_trailkeeper_collector;
pub mod zv9_trailkeeper_config;
pub mod zv9_trailkeeper_entry;
pub mod zv9_trailkeeper_macros;
pub mod zv9_trailkeeper_registry;
pub mod zv9_trailkeeper_scan;
pub mod zv9_trailkeeper_watch;

// 🧪 Placeholder test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::add(2, 2);
        assert_eq!(result, 4);
    }
}

// 🧮 Example utility
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
