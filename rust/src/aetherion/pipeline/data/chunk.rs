use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::tile::TileInfo;
use super::vector::SerializableVector2i;

/// Represents a chunk of tile data used in procedural generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MapDataChunk {
    pub chunk: HashMap<SerializableVector2i, TileInfo>,
}

impl MapDataChunk {
    pub fn new() -> Self {
        Self {
            chunk: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pos: SerializableVector2i, info: TileInfo) {
        self.chunk.insert(pos, info);
    }
}
