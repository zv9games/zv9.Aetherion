//! 🗂️ Registry Module
//! Tracks and indexes all tiles/voxels in EchoEngine.
//!
//! Supports spatial queries, mutation, and legacy-safe replay.
//! Designed for introspection, phase mutation, and contributor debugging.
//!
//! Contributors may use `upsert()`, `get()`, and `query_kind()` to interact with tile state across phases.

use std::collections::HashMap;
use crate::engine::types::{Tile, TileKind};
use crate::engine::dimension::Position;

/// Registry stores tiles indexed by their spatial position
pub struct Registry {
    pub tiles: HashMap<Position, Tile>,
}

impl Registry {
    /// 🧾 Create a new empty registry
    /// Used during `Init` or dimension flip
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// 🔁 Insert or update a tile
    /// Used during `Generate` phase or mutation rituals
    pub fn upsert(&mut self, tile: Tile) {
        self.tiles.insert(tile.position.clone(), tile);
    }

    /// 🔍 Retrieve a tile by position
    /// Used for introspection, overlays, or signal hooks
    pub fn get(&self, pos: &Position) -> Option<&Tile> {
        self.tiles.get(pos)
    }

    /// 🧹 Remove a tile and return it if it existed
    /// Used during teardown, mutation, or anomaly recovery
    pub fn remove(&mut self, pos: &Position) -> Option<Tile> {
        self.tiles.remove(pos)
    }

    /// 🧪 Query all tiles of a given kind
    /// Used for overlays, signal emission, or debug glyphs
    pub fn query_kind(&self, kind: TileKind) -> Vec<&Tile> {
        self.tiles.values().filter(|t| t.kind == kind).collect()
    }
}
