use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// This struct represents a single tile's data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileInfo {
    pub source_id: i32,
    pub atlas_coords: Vector2i,
    pub alternate_id: i32,
}

// This is the full map data chunk that the importer will produce
pub type MapDataChunk = HashMap<Vector2i, TileInfo>;