// aetherion_generate/src/generator.rs

//! Defines the Generator trait, the core interface for all procedural generation algorithms
//! in the zv9.Aetherion engine.

use aetherion_shared::chunk_data::ChunkData;
use aetherion_math::Vec2i;

/// Defines the core contract for all procedural generation algorithms.
/// Every generator (Perlin, CA, DiamondSquare, etc.) must implement this trait.
/// 
/// The `#[allow(dead_code)]` attribute suppresses warnings for traits that are
/// publicly exported but only implemented/used by downstream crates (like `aetherion_godot`).
#[allow(dead_code)] 
pub trait Generator {
    /// The unique identifier for this specific algorithm (e.g., "perlin_2d_v1").
    fn id(&self) -> &str;

    /// Generates the content for a single Chunk.
    /// It takes a Vec2i which is the world-space coordinate of the chunk.
    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData;
}