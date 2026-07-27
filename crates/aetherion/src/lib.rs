//! Aetherion — procedural generation core for Godot 4 (optional GDExtension).
//!
//! Greenfield rebuild (2026). Lineage: historic multi-crate Aetherion U8.4 proof
//! + SSXL-ext “build → deploy → launch Godot” pipeline. See `docs/LINEAGE.md`.

#![deny(missing_docs)]

pub mod chunk;
pub mod error;
pub mod version;

pub use chunk::ChunkCoord;
pub use error::AetherionError;
pub use version::{VERSION, version_string};

/// Library health check used by CLI and tests.
pub fn health() -> &'static str {
    "aetherion-ok"
}

#[cfg(feature = "godot")]
mod godot_bridge;

#[cfg(feature = "godot")]
pub use godot_bridge::*;
