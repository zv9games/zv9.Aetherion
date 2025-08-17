// registry.rs

//! 🗂️ Registry Module
//! This module tracks and indexes all tiles/voxels in the engine.
//! It supports spatial queries, mutation, and legacy-safe replay.

use std::collections::HashMap;
use crate::engine::types::{Tile, TileKind};
use crate::engine::dimension::DimensionContext;

/// Registry stores tiles indexed by their position
pub struct Registry<P> {
    pub tiles: HashMap<P, Tile<P>>,
}

impl<P: std::hash::Hash + Eq + Clone> Registry<P> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// Insert or update a tile
    pub fn upsert(&mut self, tile: Tile<P>) {
        self.tiles.insert(tile.position.clone(), tile);
    }

    /// Retrieve a tile by position
    pub fn get(&self, pos: &P) -> Option<&Tile<P>> {
        self.tiles.get(pos)
    }

    /// Remove a tile
    pub fn remove(&mut self, pos: &P) {
        self.tiles.remove(pos);
    }

    /// Query all tiles of a given kind
    pub fn query_kind(&self, kind: TileKind) -> Vec<&Tile<P>> {
        self.tiles.values().filter(|t| t.kind == kind).collect()
    }
}
