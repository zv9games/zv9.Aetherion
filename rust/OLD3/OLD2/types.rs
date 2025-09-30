use godot::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A serde-compatible wrapper for Godot's `Vector2i`
/// Used only for serialization/deserialization
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializableVector2i {
    pub x: i32,
    pub y: i32,
}

impl From<Vector2i> for SerializableVector2i {
    fn from(v: Vector2i) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<SerializableVector2i> for Vector2i {
    fn from(s: SerializableVector2i) -> Self {
        Vector2i::new(s.x, s.y)
    }
}

/// This struct represents a single tile's data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileInfo {
    pub source_id: i32,
    pub atlas_coords: SerializableVector2i,
    pub alternate_id: i32,
}

/// This is the full map data chunk that the importer will produce
pub type MapDataChunk = HashMap<SerializableVector2i, TileInfo>;
