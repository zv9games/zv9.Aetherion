//C:/ZV9/zv9.aetherion/rust/src/zv9_lib.rs


#[allow(unused_imports)]
use crate::zv9_prelude::*; // everything is in here
use godot::prelude::*; 
use godot_macros::gdextension;

pub const VERSION: &str = "0.1.0";

pub mod zv9_prelude {
    include!("zv9_prelude.rs");
}



// 🧩 Centralized init hook
pub fn init_all() {
    // Initialize logging, config, profiling, etc.
    // Example: logging::init(); config::load(); profiler::start();

    log_event!(
        EventType::System,
        "engine",
        format!("Aetherion boot sequence started (v{})", VERSION)
    );
}

// 📚 Flat module includes
pub mod zv9_aetherion_core_runtime { include!("zv9_aetherion_core_runtime.rs"); }
pub mod zv9_aetherion_core_conductor { include!("zv9_aetherion_core_conductor.rs"); }
pub mod zv9_aetherion_core_dimension { include!("zv9_aetherion_core_dimension.rs"); }
pub mod zv9_aetherion_core_lifecycle { include!("zv9_aetherion_core_lifecycle.rs"); }

pub mod zv9_aetherion_codegen_config { include!("zv9_aetherion_codegen_config.rs"); }
pub mod zv9_aetherion_codegen_dsl { include!("zv9_aetherion_codegen_dsl.rs"); }
pub mod zv9_aetherion_codegen_emitter { include!("zv9_aetherion_codegen_emitter.rs"); }
pub mod zv9_aetherion_codegen_parser { include!("zv9_aetherion_codegen_parser.rs"); }

pub mod zv9_aetherion_generator_noise { include!("zv9_aetherion_generator_noise.rs"); }
pub mod zv9_aetherion_generator_noise_config { include!("zv9_aetherion_generator_noise_config.rs"); }
pub mod zv9_aetherion_generator_patterns { include!("zv9_aetherion_generator_patterns.rs"); }
pub mod zv9_aetherion_generator_pattern_type { include!("zv9_aetherion_generator_pattern_type.rs"); }

pub mod zv9_aetherion_interaction_modifiers { include!("zv9_aetherion_interaction_modifiers.rs"); }
pub mod zv9_aetherion_interaction_tools { include!("zv9_aetherion_interaction_tools.rs"); }

pub mod zv9_aetherion_pipeline_builder_bitmask { include!("zv9_aetherion_pipeline_builder_bitmask.rs"); }
pub mod zv9_aetherion_pipeline_builder_builder { include!("zv9_aetherion_pipeline_builder_builder.rs"); }
pub mod zv9_aetherion_pipeline_builder_streamer { include!("zv9_aetherion_pipeline_builder_streamer.rs"); }
pub mod zv9_aetherion_pipeline_builder_threaded { include!("zv9_aetherion_pipeline_builder_threaded.rs"); }

pub mod zv9_aetherion_pipeline_data_chunk { include!("zv9_aetherion_pipeline_data_chunk.rs"); }
pub mod zv9_aetherion_pipeline_data_data { include!("zv9_aetherion_pipeline_data_data.rs"); }
pub mod zv9_aetherion_pipeline_data_grid { include!("zv9_aetherion_pipeline_data_grid.rs"); }
pub mod zv9_aetherion_pipeline_data_tile { include!("zv9_aetherion_pipeline_data_tile.rs"); }
pub mod zv9_aetherion_pipeline_data_vector { include!("zv9_aetherion_pipeline_data_vector.rs"); }

pub mod zv9_aetherion_structure_generation { include!("zv9_aetherion_structure_generation.rs"); }
pub mod zv9_aetherion_structure_placement { include!("zv9_aetherion_structure_placement.rs"); }

// 🔧 Godot Interface
pub mod zv9_godot_interface_api_config { include!("zv9_godot_interface_api_config.rs"); }
pub mod zv9_godot_interface_api_engine { include!("zv9_godot_interface_api_engine.rs"); }
pub mod zv9_godot_interface_api_generator { include!("zv9_godot_interface_api_generator.rs"); }
pub mod zv9_godot_interface_api_map { include!("zv9_godot_interface_api_map.rs"); }
pub mod zv9_godot_interface_api_oracle { include!("zv9_godot_interface_api_oracle.rs"); }
pub mod zv9_godot_interface_api_signals { include!("zv9_godot_interface_api_signals.rs"); }

pub mod zv9_godot_interface_bindings_godot_types { include!("zv9_godot_interface_bindings_godot_types.rs"); }
pub mod zv9_godot_interface_interface_controls { include!("zv9_godot_interface_interface_controls.rs"); }
pub mod zv9_godot_interface_interface_diagnostics { include!("zv9_godot_interface_interface_diagnostics.rs"); }
pub mod zv9_godot_interface_messaging_messages { include!("zv9_godot_interface_messaging_messages.rs"); }
pub mod zv9_godot_interface_messaging_sync { include!("zv9_godot_interface_messaging_sync.rs"); }
pub mod zv9_godot_interface_signals_definitions { include!("zv9_godot_interface_signals_definitions.rs"); }
pub mod zv9_godot_interface_signals_dispatch { include!("zv9_godot_interface_signals_dispatch.rs"); }

// 🧠 Shared
pub mod zv9_shared_grid_bounds { include!("zv9_shared_grid_bounds.rs"); }
pub mod zv9_shared_grid2d { include!("zv9_shared_grid2d.rs"); }
pub mod zv9_shared_math { include!("zv9_shared_math.rs"); }
pub mod zv9_shared_spatial { include!("zv9_shared_spatial.rs"); }
pub mod zv9_shared_traits { include!("zv9_shared_traits.rs"); }
pub mod zv9_shared_types { include!("zv9_shared_types.rs"); }

// 🧼 Util
pub mod zv9_util_config { include!("zv9_util_config.rs"); }
pub mod zv9_util_direction { include!("zv9_util_direction.rs"); }
pub mod zv9_util_logging { include!("zv9_util_logging.rs"); }
pub mod zv9_util_position { include!("zv9_util_position.rs"); }
pub mod zv9_util_profiling { include!("zv9_util_profiling.rs"); }
pub mod zv9_util_time { include!("zv9_util_time.rs"); }
pub mod zv9_util_timer { include!("zv9_util_timer.rs"); }
pub mod zv9_util_velocity { include!("zv9_util_velocity.rs"); }

// 🧭 Trailkeeper
pub mod zv9_trailkeeper_collector { include!("zv9_trailkeeper_collector.rs"); }
pub mod zv9_trailkeeper_config { include!("zv9_trailkeeper_config.rs"); }
pub mod zv9_trailkeeper_entry { include!("zv9_trailkeeper_entry.rs"); }
pub mod zv9_trailkeeper_export { include!("zv9_trailkeeper_export.rs"); }
pub mod zv9_trailkeeper_macros { include!("zv9_trailkeeper_macros.rs"); }
pub mod zv9_trailkeeper_registry { include!("zv9_trailkeeper_registry.rs"); }
pub mod zv9_trailkeeper_scan { include!("zv9_trailkeeper_scan.rs"); }
pub mod zv9_trailkeeper_watch { include!("zv9_trailkeeper_watch.rs"); }

// 🧪 Tests
#[cfg(test)]
mod integration_tests {
    // Add test modules here
}

// 🚀 Godot Extension Entry
struct AetherionEXT;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionEXT {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion is summoned.");
        }
    }
}

// 🔓 Re-export for binary access

// Prelude
pub use zv9_prelude::*;

// 🧼 Util (merged internal + public-facing)
pub mod util {
    pub use crate::zv9_util_config::*;
    pub use crate::zv9_util_direction::*;
    pub use crate::zv9_util_logging::*;
    pub use crate::zv9_util_position::*;
    pub use crate::zv9_util_profiling::*;
    pub use crate::zv9_util_time::*;
    pub use crate::zv9_util_timer::*;
    pub use crate::zv9_util_velocity::*;

    pub mod logging {
        pub use crate::zv9_util_logging::*;
    }
}

// 🧭 Trailkeeper (merged internal + public-facing)
pub mod trailkeeper {
    pub use crate::zv9_trailkeeper_collector::*;
    pub use crate::zv9_trailkeeper_config::*;
    pub use crate::zv9_trailkeeper_entry::*;
    pub use crate::zv9_trailkeeper_export::*;
    //pub use crate::zv9__trailkeeper__macros::*;
    pub use crate::zv9_trailkeeper_registry::*;
    pub use crate::zv9_trailkeeper_scan::*;
    pub use crate::zv9_trailkeeper_watch::*;

    pub mod collector {
        pub use crate::zv9_trailkeeper_collector::*;
    }
    pub mod config {
        pub use crate::zv9_trailkeeper_config::*;
    }
    pub mod scan {
        pub use crate::zv9_trailkeeper_scan::*;
    }
    pub mod entry {
        pub use crate::zv9_trailkeeper_entry::*;
    }
}


pub mod core {
    pub mod runtime {
        pub use crate::zv9_aetherion_core_runtime::start;
    }

    pub use crate::zv9_aetherion_core_conductor::*;
    pub use crate::zv9_aetherion_core_dimension::*;
    pub use crate::zv9_aetherion_core_lifecycle::*;
}

pub mod pipeline_builder {
    pub mod bitmask {
        pub use crate::zv9_aetherion_pipeline_builder_bitmask::*;
    }
}


// 🧠 Core types
pub use zv9_aetherion_core_conductor::{Conductor, ProcCommand};
pub use zv9_aetherion_pipeline_data_chunk::MapDataChunk;
pub use zv9_godot_interface_messaging_sync::GodotSync;
pub use zv9_aetherion_core_runtime::start;
pub use zv9_aetherion_pipeline_builder_bitmask::*;
