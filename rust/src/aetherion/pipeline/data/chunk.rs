use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{tile::TileInfo, vector::SerializableVector2i};

/// A chunk of tile data used in procedural generation.
/// Maps grid positions to tile metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MapDataChunk {
    pub tiles: HashMap<SerializableVector2i, TileInfo>,
}

impl MapDataChunk {
    /// Creates an empty chunk.
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// Inserts a tile at the given position.
    pub fn insert(&mut self, pos: SerializableVector2i, info: TileInfo) {
        self.tiles.insert(pos, info);
    }

    /// Returns the number of tiles in the chunk.
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns true if the chunk is empty.
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Consumes the chunk and returns its inner map.
    pub fn into_inner(self) -> HashMap<SerializableVector2i, TileInfo> {
        self.tiles
    }

    /// Returns an iterator over all tile entries.
    pub fn iter(&self) -> impl Iterator<Item = (&SerializableVector2i, &TileInfo)> {
        self.tiles.iter()
    }

    /// Merges another chunk into this one, overwriting overlapping positions.
    pub fn merge(&mut self, other: MapDataChunk) {
        self.tiles.extend(other.tiles);
    }

    /// Returns a reference to a tile at the given position, if it exists.
    pub fn get(&self, pos: &SerializableVector2i) -> Option<&TileInfo> {
        self.tiles.get(pos)
    }
}
