use godot::prelude::*;
use aetherion_core::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};

/// 🧩 Extension trait for converting MapDataChunk to Godot Dictionary.
pub trait MapDataChunkExt {
    fn to_dictionary(&self) -> Dictionary;
}

impl MapDataChunkExt for MapDataChunk {
    fn to_dictionary(&self) -> Dictionary {
        let mut dict = Dictionary::new();


        for (pos, tile) in &self.tiles {
            let key = Vector2i::new(pos.x, pos.y).to_variant();

            let mut tile_dict = Dictionary::new();
            tile_dict.insert("source_id", tile.source_id);
            tile_dict.insert("atlas_coords", Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y));
            tile_dict.insert("alternate_id", tile.alternate_id);
            tile_dict.insert("rotation", tile.rotation);
            tile_dict.insert("layer", tile.layer);
            tile_dict.insert("flags", tile.flags);

            if let Some(variant_id) = tile.variant_id {
                tile_dict.insert("variant_id", variant_id);
            }
            if let Some(frame_count) = tile.frame_count {
                tile_dict.insert("frame_count", frame_count);
            }
            if let Some(animation_speed) = tile.animation_speed {
                tile_dict.insert("animation_speed", animation_speed);
            }

            dict.insert(key, tile_dict);
        }

        dict
    }
}
