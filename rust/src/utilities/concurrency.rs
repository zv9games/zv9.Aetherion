use godot::prelude::*;
use std::thread;
pub use crossbeam_channel::{Receiver, Sender};

use crate::data_processing::types::{MapDataChunk, TileInfo, SerializableVector2i};

// The type of message we will send through our channel
pub struct TilePlacementMessage {
    pub position: Vector2i,
    pub info: TileInfo,
}

/// Spawns a worker thread to process tile placement and stream messages to the main thread.
pub fn spawn_tile_placement_task(
    tile_data: MapDataChunk,
    tilemap: Gd<godot::classes::TileMap>,
) -> Receiver<TilePlacementMessage> {
    let (tx, rx) = crossbeam_channel::unbounded::<TilePlacementMessage>();

    thread::spawn(move || {
        let total_tiles = tile_data.len();
        let mut placed_count = 0;

        for (pos, info) in tile_data.iter() {
            let godot_pos: Vector2i = pos.clone().into(); // ✅ Clone before converting

            if tx.send(TilePlacementMessage {
                position: godot_pos,
                info: info.clone(),
            }).is_err() {
                break; // Receiver dropped
            }

            placed_count += 1;

            if placed_count % 10_000 == 0 {
                // Future: emit progress signal via thread-safe Godot handle
            }
        }

        godot_print!("Worker thread finished processing {} tiles.", placed_count);
    });

    rx
}
