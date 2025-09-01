use godot::prelude::*;
use crate::aetherion::pipeline::data::{MapDataChunk, TileInfo};

/// AetherionMap — exposes runtime tile/voxel state to Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AetherionMap {
    pub chunk: Option<MapDataChunk>,
}

#[godot_api]
impl AetherionMap {
    #[func]
    fn _ready(&self) {
        godot_print!("🧩 AetherionMap initialized.");
    }

    /// Load a chunk from raw tile data
    #[func]
    fn load_chunk(&mut self, tiles: Array) {
        let mut tile_vec = Vec::new();
        for tile in tiles.iter_shared() {
            if let Some(dict) = tile.try_to::<Dictionary>() {
                let id = dict.get("id").try_to::<i32>().unwrap_or(0);
                let meta = dict.get("meta").try_to::<String>().unwrap_or_default();
                let visible = dict.get("visible").try_to::<bool>().unwrap_or(true);
                let layer = dict.get("layer").try_to::<i32>().unwrap_or(0);
                tile_vec.push(TileInfo {
                    id,
                    meta,
                    visible,
                    layer,
                });
            }
        }
        self.chunk = Some(MapDataChunk::from_tiles(tile_vec));
    }

    /// Get tile info at index
    #[func]
    fn get_tile(&self, index: i32) -> Dictionary {
        let mut dict = Dictionary::new();
        if let Some(chunk) = &self.chunk {
            if let Some(tile) = chunk.tiles.get(index as usize) {
                dict.insert("id", tile.id);
                dict.insert("meta", tile.meta.clone());
                dict.insert("visible", tile.visible);
                dict.insert("layer", tile.layer);
            }
        }
        dict
    }

    /// Clear the current chunk
    #[func]
    fn clear_chunk(&mut self) {
        self.chunk = None;
    }
}
