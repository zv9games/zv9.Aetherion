//! Core generation algorithms, runtime orchestration, and task management.
//!
//! This crate defines the `Generator` trait and orchestrates asynchronous world
//! generation via the `Conductor`.

// -------------------------------------------------------------------------------------------------
// MODULE DECLARATIONS
// -------------------------------------------------------------------------------------------------

pub mod conductor;
pub mod benchmark_logic;
pub mod perlin_generator;
pub mod cellular_automata_generator;

// -------------------------------------------------------------------------------------------------
// CORE TRAIT DEFINITION (Generator Interface)
// -------------------------------------------------------------------------------------------------

use aetherion_shared::chunk_data::ChunkData;
use aetherion_math::Vec2i;
use tracing::{info, error};

/// Defines the core contract for all procedural generation algorithms.
/// Every generator (Perlin, CA, DiamondSquare, etc.) must implement this trait.
pub trait Generator {
    /// The unique identifier for this specific algorithm (e.g., "perlin_2d_v1").
    fn id(&self) -> &str;

    /// Generates the content for a single Chunk.
    /// It takes a Vec2i which is the world-space coordinate of the chunk.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}

// -------------------------------------------------------------------------------------------------
// PUBLIC EXPORTS (Used by aetherion_godot and aetherion_cli)
// -------------------------------------------------------------------------------------------------

// Re-export core structs and config
pub use conductor::{Conductor, GeneratorConfig};
pub use perlin_generator::PerlinGenerator;
pub use cellular_automata_generator::CellularAutomataGenerator;

// Re-export utility functions
pub use benchmark_logic::benchmark_generation_workload;


// -------------------------------------------------------------------------------------------------
// PUBLIC API FOR CLI/FFI (Validation Entry Points)
// -------------------------------------------------------------------------------------------------

/// Starts the Aetherion Runtime, creating and immediately shutting down the Conductor.
///
/// NOTE: This is the **structural validation test for CLI Menu [4]** (Start Runtime).
pub fn start_runtime_placeholder() {
    // Pass None as the config_path argument to satisfy the updated Conductor::new signature.
    match Conductor::new(None) {
        // FIX: Properly destructure the 3-element tuple (Conductor, ConductorState, Receiver).
        Ok((conductor, _state, _receiver)) => {
            info!("Runtime created successfully. Testing immediate graceful teardown...");
            
            // Call the consuming teardown method.
            conductor.graceful_teardown();
        }
        Err(e) => {
            error!("Failed to initialize Conductor/Runtime: {:?}", e);
        }
    }
}