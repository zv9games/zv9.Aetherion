use godot::builtin::{Array, Dictionary, Vector2i, Variant};
use godot::classes::{Node, SceneTree, TileMap};
use godot::meta::AsArg;
use godot::obj::WithBaseField;
use godot::prelude::*;
use aetherion_core::log_component;

#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🧩 AetherionMap — Godot-facing node for chunk loading and tile inspection.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionMap {
    #[base]
    base: Base<Node>,
    pub chunk: Option<MapDataChunk>,
    pub tilemap: Option<Gd<TileMap>>,
}

#[godot_api]
impl AetherionMap {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            chunk: None,
            tilemap: None,
        }
    }

    #[func]
    fn _ready(&self) {
        godot_print!("🧩 AetherionMap initialized.");
        log_component!("AetherionMap", "Node for chunk loading and tile inspection");
    }

    /// 🧱 Assigns a TileMap for rendering.
    #[func]
    fn set_tilemap(&mut self, tilemap: Gd<TileMap>) {
        self.tilemap = Some(tilemap);
        godot_print!("🧩 TileMap assigned to AetherionMap.");
    }

    /// 🧩 Loads a chunk from raw tile data and renders it.
    #[func]
    fn load_chunk(&mut self, tiles: Array<Variant>) {
        let mut chunk = MapDataChunk::new();

        for (i, tile_variant) in tiles.iter_shared().enumerate() {
            if let Ok(dict) = tile_variant.try_to::<Dictionary>() {
                let atlas_vec = dict.get("atlas_coords")
                    .and_then(|v| v.try_to::<Vector2i>().ok())
                    .unwrap_or(Vector2i::ZERO);

                let tile = TileInfo {
                    source_id: dict.get("source_id").and_then(|v| v.try_to::<i32>().ok()).unwrap_or(0),
                    atlas_coords: SerializableVector2i { x: atlas_vec.x, y: atlas_vec.y },
                    alternate_id: dict.get("alternate_id").and_then(|v| v.try_to::<i32>().ok()).unwrap_or(0),
                    rotation: dict.get("rotation").and_then(|v| v.try_to::<i32>().ok()).map(|v| v.clamp(0, u8::MAX as i32) as u8).unwrap_or(0),
                    layer: dict.get("layer").and_then(|v| v.try_to::<i32>().ok()).map(|v| v.clamp(0, u8::MAX as i32) as u8).unwrap_or(0),
                    flags: 0,
                    variant_id: None,
                    frame_count: None,
                    animation_speed: None,
                };

                let key = SerializableVector2i { x: i as i32, y: 0 };
                chunk.insert(key, tile.clone());

                if let Some(tilemap) = self.tilemap.as_mut() {
                    let pos = Vector2i::new(key.x, key.y);
                    let atlas = Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y);

                    tilemap.set_cell_ex(0, pos)
                        .source_id(tile.source_id)
                        .atlas_coords(atlas)
                        .alternative_tile(tile.alternate_id)
                        .done();
                }
            }
        }

        self.chunk = Some(chunk);
        godot_print!("🧩 Chunk loaded and rendered with {} tiles.", tiles.len());
    }

    /// 🔍 Retrieves tile info at the given index.
    #[func]
    fn get_tile(&self, index: i32) -> Dictionary {
        let mut dict = Dictionary::new();

        if let Some(chunk) = &self.chunk {
            let key = SerializableVector2i { x: index, y: 0 };
            if let Some(tile) = chunk.get(&key) {
                dict.insert("source_id", tile.source_id);
                dict.insert("atlas_coords", Vector2i::new(tile.atlas_coords.x, tile.atlas_coords.y));
                dict.insert("alternate_id", tile.alternate_id);
                dict.insert("rotation", tile.rotation);
                dict.insert("layer", tile.layer);
            } else {
                godot_warn!("🧩 No tile found at index {}", index);
            }
        } else {
            godot_warn!("🧩 No chunk loaded when requesting tile {}", index);
        }

        dict
    }

    /// 🧹 Clears the currently loaded chunk and TileMap.
    #[func]
    fn clear_chunk(&mut self) {
        self.chunk = None;
        if let Some(tilemap) = self.tilemap.as_mut() {
            tilemap.clear();
        }
        godot_print!("🧩 Chunk and TileMap cleared.");
    }

    /// 🧪 Simulates generation and placement of a test chunk.
    #[func]
    fn test_chunk_placement(&mut self) {
        let tree: Gd<SceneTree> = self.base().get_tree().unwrap();
        let root = tree.get_root().unwrap();
        let tilemap: Gd<TileMap> = root.get_node_as("aetheriontester/main/expansive_tilemap");

        self.set_tilemap(tilemap);

        let mut tiles = Array::new();
        for i in 0..100 {
            let mut dict = Dictionary::new();
            dict.insert("source_id", 0);
            dict.insert("atlas_coords", Vector2i::new(i % 8, i / 8));
            dict.insert("alternate_id", 0);
            dict.insert("rotation", 0);
            dict.insert("layer", 0);
            tiles.push(dict.to_variant().into_arg());
        }

        self.load_chunk(tiles);
        godot_print!("✅ Test chunk delivered to AetherionMap.");
    }
}
