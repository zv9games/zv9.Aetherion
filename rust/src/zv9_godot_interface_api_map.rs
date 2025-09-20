//C:/ZV9/zv9.aetherion/rust/src/zv9_godot_interface_api_map.rs

use godot::prelude::*;
use godot::builtin::{Array, Dictionary, Vector2i};
use godot::classes::Node;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🧩 AetherionMap — Godot-facing node for chunk loading and tile inspection.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionMap {
    pub chunk: Option<MapDataChunk>,
}

#[godot_api]
impl AetherionMap {
	#[allow(dead_code)]
    fn init(_base: Base<Node>) -> Self {
        Self { chunk: None }
    }

    #[func]
    fn _ready(&self) {
		godot_print!("🧩 AetherionMap initialized.");
		log_component!("AetherionMap", "Node for chunk loading and tile inspection");
	}

    /// Loads a chunk from raw tile data, skipping invalid entries.
    #[func]
    fn load_chunk(&mut self, tiles: Array<Variant>) {
        let mut chunk = MapDataChunk::new();

        for (i, tile_variant) in tiles.iter_shared().enumerate() {
            if let Ok(dict) = tile_variant.try_to::<Dictionary>() {
                let source_id = dict.get("source_id").and_then(|v| v.try_to::<i32>().ok()).unwrap_or(0);
                let atlas_coords = dict.get("atlas_coords").and_then(|v| v.try_to::<Vector2i>().ok()).unwrap_or(Vector2i::ZERO).into();
                let alternate_id = dict.get("alternate_id").and_then(|v| v.try_to::<i32>().ok()).unwrap_or(0);
                let rotation = dict.get("rotation").and_then(|v| v.try_to::<i32>().ok()).map(|v| v.clamp(0, u8::MAX as i32) as u8).unwrap_or(0);
                let layer = dict.get("layer").and_then(|v| v.try_to::<i32>().ok()).map(|v| v.clamp(0, u8::MAX as i32) as u8).unwrap_or(0);

                let tile = TileInfo {
                    source_id,
                    atlas_coords,
                    alternate_id,
                    rotation,
                    layer,
                    flags: 0,
                    variant_id: None,
                    frame_count: None,
                    animation_speed: None,
                };

                let key = SerializableVector2i::from(Vector2i::new(i as i32, 0));
                chunk.tiles.insert(key, tile);
            }
        }

        self.chunk = Some(chunk);
        godot_print!("🧩 Chunk loaded with {} tiles.", tiles.len());
    }

    /// Retrieves tile info at the given index.
    #[func]
    fn get_tile(&self, index: i32) -> Dictionary {
        let mut dict = Dictionary::new();

        if let Some(chunk) = &self.chunk {
            let key = SerializableVector2i::from(Vector2i::new(index, 0));
            if let Some(tile) = chunk.tiles.get(&key) {
                let _ = dict.insert("source_id", tile.source_id);
                let _ = dict.insert("atlas_coords", Vector2i::from(tile.atlas_coords));
                let _ = dict.insert("alternate_id", tile.alternate_id);
                let _ = dict.insert("rotation", tile.rotation);
                let _ = dict.insert("layer", tile.layer);
            } else {
                godot_warn!("🧩 No tile found at index {}", index);
            }
        } else {
            godot_warn!("🧩 No chunk loaded when requesting tile {}", index);
        }

        dict
    }

    /// Clears the currently loaded chunk.
    #[func]
    fn clear_chunk(&mut self) {
        self.chunk = None;
        godot_print!("🧩 Chunk cleared.");
    }
}



// the end