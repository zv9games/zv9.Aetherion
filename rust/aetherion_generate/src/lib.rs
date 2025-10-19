// aetherion_generate/src/lib.rs
//! Core generation algorithms, runtime orchestration, and task management.

// -------------------------------------------------------------------------------------------------
// MODULE EXPOSURE
// -------------------------------------------------------------------------------------------------
// Expose the Conductor (the validated Runtime/Orchestration core).
pub mod conductor;
// Expose the Generator trait and its implementations (the MVG priority).
mod generator;
mod perlin_generator;
mod cellular_automata_generator;


// -------------------------------------------------------------------------------------------------
// CORE TRAIT DEFINITION (Generator Interface)
// -------------------------------------------------------------------------------------------------
use aetherion_shared::chunk_data::ChunkData;
use aetherion_math::Vec2i;

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
// PUBLIC EXPORTS
// -------------------------------------------------------------------------------------------------
// Re-export the main components for easy use by other crates (aetherion_godot, aetherion_cli).
pub use conductor::Conductor;

// -------------------------------------------------------------------------------------------------
// PUBLIC API FOR CLI/FFI (Validation Entry Points)
// -------------------------------------------------------------------------------------------------
use tracing::{info, error};

/// Starts the Aetherion Runtime, creating and immediately shutting down the Conductor.
/// 
/// NOTE: This is the **structural validation test for CLI Menu [4]** (Start Runtime).
pub fn start_runtime_placeholder() {
    match Conductor::new() {
        Ok(conductor) => {
            info!("Runtime created successfully. Testing immediate graceful shutdown...");
            // The call now correctly consumes the `conductor` instance.
            conductor.shutdown(); 
        }
        Err(e) => {
            error!("Failed to initialize Conductor/Runtime: {:?}", e);
        }
    }
}