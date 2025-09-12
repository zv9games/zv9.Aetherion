
/// ✅ Suggestions for aetherion/pipeline/data/map_data_chunk.rs

// 🔧 Add tile removal and mutation methods:
//     - `fn remove(&mut self, pos: &SerializableVector2i)`
//     - `fn update(&mut self, pos: &SerializableVector2i, info: TileInfo)`
//     - Useful for in-game editing and procedural mutation

// 🧩 Add spatial utilities:
//     - `fn bounds(&self) -> Option<(SerializableVector2i, SerializableVector2i)>`
//     - Enables chunk visualization, collision, or export

// 🚦 Add validation or filtering:
//     - `fn filter_by_layer(&self, layer: u8) -> Vec<(&SerializableVector2i, &TileInfo)>`
//     - Useful for rendering or selective processing

// 📚 Document serialization expectations:
//     - Clarify how `to_dictionary()` maps to Godot types
//     - Note any assumptions about `TileInfo` structure

// 🧪 Add unit tests for merge, insert, and dictionary conversion:
//     - Validate overwrites, empty states, and Godot compatibility

// 🧼 Optional: Add debug summary or pretty-print:
//     - `fn summary(&self) -> String`
//     - Useful for diagnostics or editor overlays

// 🚀 Future: Add chunk compression or diffing:
//     - e.g. `fn diff(&self, other: &Self) -> MapDataChunk`
//     - Enables efficient network sync or undo/redo

// 🧠 Consider exposing tile metadata queries:
//     - e.g. `fn count_by_flag(&self, flag: u32) -> usize`
//     - Useful for gameplay logic or analytics


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use godot::prelude::*;
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

    /// Converts the chunk into a Godot Dictionary for signal dispatch.
    pub fn to_dictionary(&self) -> Dictionary {
		let mut dict = Dictionary::new();
		let mut tile_data = Dictionary::new();

		for (pos, info) in &self.tiles {
			let _ = tile_data.insert(pos.to_vector2i(), info.to_dictionary());
		}

		let _ = dict.insert("tile_count", self.len() as i32);
		let _ = dict.insert("tiles", tile_data);

		dict
	}

}
