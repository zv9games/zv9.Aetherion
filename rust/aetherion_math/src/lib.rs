// aetherion_math/src/lib.rs

//! The Foundation Layer crate for all spatial types, vector math,
//! coordinate systems, and deterministic hashing algorithms.

// Re-export the core types from the shared foundation
pub use aetherion_shared::prelude::*;

// --- MODULE DEFINITIONS ---
// P1 Task: Implement World-to-Chunk key mapping.
pub mod coordinate_system;

// NEW: This module contains generation-related math utilities, including 
// the logic for 'process_data'. It replaces 'spatial_types' to align 
// with the file 'generation_utils.rs'.
pub mod generation_utils; 

// Structural Fix: Aligns with the file 'hashing.rs' in the directory.
pub mod hashing; 

// --- EXPORTS FOR DOWNSTREAM CRATES ---

// CRITICAL FIX FOR E0432:
// Expose the required function/struct for 'aetherion_generate' to find it.
pub use crate::generation_utils::process_data;

/// Math-specific prelude for convenient imports in downstream crates.
pub mod prelude {
	pub use super::coordinate_system::*;
	pub use super::generation_utils::*;
	pub use super::hashing::*;
}

pub fn initialize_math_system() {
	// Assuming 'tracing' is added to Cargo.toml as discussed previously.
	tracing::info!("Aetherion Math system initialized and ready.");
}