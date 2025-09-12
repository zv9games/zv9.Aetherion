//C:/ZV9/zv9.aetherion/rust/src/godot4/api/map.rs


/// ✅ Suggestions for godot4/api/map.rs

// 🔧 Add support for multi-row chunk loading:
//     - Currently assumes tiles are laid out in a single row (y = 0)
//     - Consider parsing `x`, `y` from each tile’s dictionary or using a 2D index

// 🧩 Add validation and error reporting:
//     - Emit warnings for malformed tile dictionaries or missing fields
     - Could return a result or signal with error count

// 🚦 Add bounds checking for `get_tile()`:
//     - Prevent out-of-range access or negative indices
     - Could return `null` or a default tile dictionary

// 📚 Document expected tile format for `load_chunk()`:
//     - Clarify required keys and types in the input `Array<Variant>`
//     - Could include a sample dictionary in comments

// 🧪 Add unit tests or GDScript integration tests:
//     - Validate chunk loading, tile retrieval, and clearing behavior
     - Ensure dictionary conversion matches `TileInfo::to_dictionary()`

// 🧼 Optional: Add chunk summary or metadata access:
//     - `fn get_tile_count() -> i32`
//     - `fn get_bounds() -> Rect2i`
//     - Useful for editor overlays or diagnostics

// 🚀 Future: Add support for tile mutation or editing:
//     - `fn set_tile(index: i32, tile: Dictionary)`
//     - Enables runtime editing or procedural updates

// 🧠 Consider exposing chunk serialization:
//     - `fn export_chunk() -> Array<Variant>`
//     - Enables saving, syncing, or debugging tile data


use godot::prelude::*;
use godot::builtin::{Array, Dictionary, Vector2i};
use godot::classes::Node;

use crate::aetherion::pipeline::data::{MapDataChunk, TileInfo, SerializableVector2i};

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
    }

    /// Loads a chunk from raw tile data, skipping invalid entries.
    #[func]
    fn load_chunk(&mut self, tiles: Array<Variant>) {
        let mut chunk = MapDataChunk::new();

        for (i, tile_variant) in tiles.iter_shared().enumerate() {
            if let Ok(dict) = tile_variant.try_to::<Dictionary>() {
                let source_id = dict
                    .get("source_id")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);

                let atlas_coords = dict
                    .get("atlas_coords")
                    .and_then(|v| v.try_to::<Vector2i>().ok())
                    .unwrap_or(Vector2i::ZERO)
                    .into();

                let alternate_id = dict
                    .get("alternate_id")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);

                let rotation = dict
                    .get("rotation")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .map(|v| v.clamp(0, u8::MAX as i32) as u8)
                    .unwrap_or(0);

                let layer = dict
                    .get("layer")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .map(|v| v.clamp(0, u8::MAX as i32) as u8)
                    .unwrap_or(0);

                let tile = TileInfo {
                    source_id,
                    atlas_coords,
                    alternate_id,
                    rotation,
                    layer,
                    flags: 0,
                };

                let key = SerializableVector2i::from(Vector2i::new(i as i32, 0));
                chunk.tiles.insert(key, tile);
            }
        }

        self.chunk = Some(chunk);
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
				godot_warn!("No tile found at index {}", index);
			}
		} else {
			godot_warn!("No chunk loaded when requesting tile {}", index);
		}

		dict
	}

    #[func]
    fn clear_chunk(&mut self) {
        self.chunk = None;
        godot_print!("Chunk cleared.");
    }
}

//end map.rs