use godot::prelude::*;
use godot::classes::TileMap;
use crossbeam_channel::Sender;
use crate::data_processing::types::{MapDataChunk, TileInfo};

pub fn place_tiles(
    tile_data: MapDataChunk, 
    tilemap: Gd<TileMap>,
    tx: Sender<(Vector2i, TileInfo)>
) {
    let total_tiles = tile_data.len();
    let mut placed_count = 0;

    for (pos, info) in tile_data.iter() {
        // Convert SerializableVector2i into Vector2i before sending
        let godot_pos: Vector2i = pos.clone().into();

        if tx.send((godot_pos, info.clone())).is_err() {
            godot_print!("Error sending tile placement message to the main thread.");
            break;
        }

        placed_count += 1;
        if placed_count % 100000 == 0 {
            let progress = placed_count as f32 / total_tiles as f32;
            // You can emit a signal or log progress here
        }
    }

    godot_print!("Placement complete – {} tiles processed by the worker thread.", placed_count);
}
