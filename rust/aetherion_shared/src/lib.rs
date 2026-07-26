//! Aetherion Shared — Foundation Layer (P1)
//! Core data structures, error handling, and engine-wide primitives.
//! For the hopeless wanderers: this is where worlds begin.

pub mod chunk_data;
pub mod tile_data;
pub mod grid_bounds;
pub mod tile_type;
pub mod errors;

// ── Core Exports ─────────────────────────────────────────────────────────────
pub use chunk_data::ChunkData;
pub use tile_data::TileData;
pub use grid_bounds::GridBounds;
pub use tile_type::TileType;
pub use errors::{AetherionError, AetherionResult};

// ── Engine Lifecycle ────────────────────────────────────────────────────────
/// Initializes shared data and global constants.
/// Called once at engine startup via FFI or CLI.
pub fn initialize_shared_data() {
    tracing::info!("Aetherion Shared: Foundation Layer (P1) initialized. Ready for generation.");
}

// ── Error Propagation (anyhow) ───────────────────────────────────────────────
pub use anyhow;

// ── Legacy Compatibility (To Be Removed in v9.0) ─────────────────────────────
/// Legacy data container — used by early cache prototypes.
/// **DEPRECATED**: Will be removed in Aetherion v9.0.
/// Use `ChunkData` + `TileData` instead.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AetherionData {
    pub id: u64,
    pub timestamp: u64,
    pub value: String,
}

// ── Prelude: One-Stop Import for All Crates ─────────────────────────────────
/// Import this in other crates for instant access to core types.
pub mod prelude {
    pub use super::chunk_data::ChunkData;
    pub use super::tile_data::TileData;
    pub use super::grid_bounds::GridBounds;
    pub use super::tile_type::TileType;
    pub use super::errors::{AetherionError, AetherionResult};
}