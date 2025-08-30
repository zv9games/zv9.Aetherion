use godot::prelude::*;
use godot::meta::{GodotConvert, ToGodot, FromGodot};
use godot::meta::error::ConvertError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A serde-compatible wrapper for Godot's `Vector2i`.
/// Used for serializing and deserializing data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

// Core Godot conversion trait
impl GodotConvert for SerializableVector2i {
    type Via = Vector2i; // Specify the intermediate Godot type
}

// Convert to Godot (to Variant via Vector2i)
impl ToGodot for SerializableVector2i {
    type ToVia<'v> = Vector2i; // Include lifetime parameter 'v

    fn to_godot(&self) -> Self::ToVia<'_> {
        Vector2i::new(self.x, self.y)
    }
}

// Convert from Godot (from Vector2i to Self)
impl FromGodot for SerializableVector2i {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(SerializableVector2i {
            x: via.x,
            y: via.y,
        })
    }
}

/// Represents a single tile's data for Godot 4.x.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileInfo {
    pub source_id: i32,
    pub atlas_coords: SerializableVector2i,
    pub alternate_id: i32,
}

/// Full map data chunk produced by the importer.
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