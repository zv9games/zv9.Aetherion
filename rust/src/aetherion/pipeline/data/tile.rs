use serde::{Deserialize, Serialize};
use super::vector::SerializableVector2i;

/// Metadata for a single tile in the map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileInfo {
    pub source_id: i32,
    pub atlas_coords: SerializableVector2i,
    pub alternate_id: i32,
}
