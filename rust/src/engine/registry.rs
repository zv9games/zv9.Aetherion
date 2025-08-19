//! 🗂️ Registry Module
//! This module tracks and indexes all tiles/voxels in the engine.
//! It supports spatial queries, mutation, and legacy-safe replay.

use std::collections::HashMap;
use crate::engine::types::{Tile, TileKind};
use crate::engine::dimension::Position; // ← Direct import of Position

/// Registry stores tiles indexed by their position
pub struct Registry {
    pub tiles: HashMap<Position, Tile>,
}

impl Registry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// Insert or update a tile
    pub fn upsert(&mut self, tile: Tile) {
        self.tiles.insert(tile.position.clone(), tile);
    }

    /// Retrieve a tile by position
    pub fn get(&self, pos: &Position) -> Option<&Tile> {
        self.tiles.get(pos)
    }

    /// Remove a tile and return it if it existed
    pub fn remove(&mut self, pos: &Position) -> Option<Tile> {
        self.tiles.remove(pos)
    }

    /// Query all tiles of a given kind
    pub fn query_kind(&self, kind: TileKind) -> Vec<&Tile> {
        self.tiles.values().filter(|t| t.kind == kind).collect()
    }
}
