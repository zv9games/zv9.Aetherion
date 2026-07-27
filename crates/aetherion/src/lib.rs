//! Aetherion — procedural generation core for Godot 4 (optional GDExtension).
//!
//! Greenfield rebuild (2026). Lineage: historic multi-crate Aetherion U8.4 proof
//! + SSXL-ext “build → deploy → launch Godot” pipeline. See `docs/LINEAGE.md`.

#![deny(missing_docs)]

pub mod chunk;
pub mod conductor;
pub mod error;
pub mod generate;
pub mod version;

pub use chunk::{ChunkCoord, ChunkData};
pub use conductor::{run_region, GenerationReport};
pub use error::AetherionError;
pub use generate::FillMode;
pub use version::{version_string, VERSION};

/// Library health check used by CLI and tests.
pub fn health() -> &'static str {
    "aetherion-ok"
}

#[cfg(feature = "godot")]
mod godot_bridge;
#[cfg(feature = "godot")]
mod host_multimesh;
#[cfg(feature = "godot")]
mod host_tilemap;

#[cfg(feature = "godot")]
pub use godot_bridge::*;
