//C:/ZV9/zv9.aetherion/rust/src/zv9_godot_interface_api_map.rs
use godot::builtin::{Array, Dictionary, Vector2i, Variant};
use godot::classes::{Node, SceneTree, TileMap};
use godot::meta::AsArg;
use godot::obj::WithBaseField;
use godot::prelude::*;

#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// 🧩 AetherionMap — Godot-facing node for chunk loading and tile inspection.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionMap {
    /// This field powers `self.base()` via the WithBaseField trait.
    #[base]
    base: Base<Node>,

    pub chunk: Option<MapDataChunk>,
    pub tilemap: Option<Gd<TileMap>>,
}

#[godot_api]
impl AetherionMap {
	#[allow(dead_code)]
    // The macro will pass you the Base<Node> here.
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
                    variant_id: None,
                    frame_count: None,
                    animation_speed: None,
                };

                let key = SerializableVector2i::from(Vector2i::new(i as i32, 0));
                chunk.tiles.insert(key, tile.clone());

                if let Some(tilemap) = self.tilemap.as_mut() {
                    tilemap
                        .set_cell_ex(0, key.into())
                        .source_id(tile.source_id)
                        .atlas_coords(tile.atlas_coords.into())
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
        // Now `self.base()` is available thanks to the #[base] field above:
        let tree: Gd<SceneTree> = self.base().get_tree().unwrap();
        let root = tree.get_root().unwrap(); // the root Viewport inherits Node

        // Lookup the TileMap in your scene (panics if the path is wrong)
        let tilemap: Gd<TileMap> =
            root.get_node_as::<TileMap>("aetheriontester/main/expansive_tilemap");

        self.set_tilemap(tilemap);

        let mut tiles = Array::new();
        for i in 0..100 {
            let mut dict = Dictionary::new();
            let _ = dict.insert("source_id",    0);
            let _ = dict.insert("atlas_coords", Vector2i::new(i % 8, i / 8));
            let _ = dict.insert("alternate_id", 0);
            let _ = dict.insert("rotation",     0);
            let _ = dict.insert("layer",        0);

            // `.into_arg()` lifts this Variant to ByValue so Array::push will accept it
            tiles.push(dict.to_variant().into_arg());
        }

        self.load_chunk(tiles);
        godot_print!("✅ Test chunk delivered to AetherionMap.");
    }
}


// the end