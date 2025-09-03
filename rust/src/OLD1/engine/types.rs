// types.rs

//! 🧱 Types Module
//! Defines core data structures for EchoEngine.
//! Dimension-agnostic, debug-friendly, and legacy-safe.

use crate::engine::dimension::Position;

/// Canonical tile kinds
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TileKind {
    Floor,
    Wall,
    Void,
    Spawn,
    Exit,
    Custom(String), // For modded or symbolic tiles
}

/// Core tile structure
#[derive(Clone, Debug)]
pub struct Tile {
    pub position: Position,
    pub kind: TileKind,
    pub metadata: Option<TileMeta>,
}

/// Optional metadata for tiles
#[derive(Clone, Debug)]
pub struct TileMeta {
    pub label: Option<String>,
    pub tags: Vec<String>,
    pub created_at: u64, // Tick timestamp
}
