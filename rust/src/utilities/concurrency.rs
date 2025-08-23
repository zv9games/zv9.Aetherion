// In utilities/concurrency.rs
use godot::prelude::*;
use std::thread;
use crossbeam_channel::{self, Sender, Receiver};

// The type of message we will send through our channel
pub struct TilePlacementMessage {
    pub position: Vector2i,
    pub info: crate::data_processing::types::TileInfo,
}

// A helper function to create the channel and spawn the task
// This function will be called from `godot_bridge/commands.rs`
pub fn spawn_tile_placement_task(
    tile_data: crate::data_processing::types::MapDataChunk,
    tilemap: Gd<godot::classes::TileMap>
) -> Receiver<TilePlacementMessage> {
    // Create a multi-producer, single-consumer channel.
    let (tx, rx) = crossbeam_channel::unbounded::<TilePlacementMessage>();

    // Spawn a new thread to run the `tile_placer` logic
    thread::spawn(move || {
        let total_tiles = tile_data.len();
        let mut placed_count = 0;

        for (pos, info) in tile_data.iter() {
            // Send the message through the channel
            if tx.send(TilePlacementMessage { position: *pos, info: info.clone() }).is_err() {
                // The receiver has been dropped, so we can stop.
                break;
            }

            placed_count += 1;
            // Periodically check for progress and send a signal
            if placed_count % 10000 == 0 {
                // Here is where we'll emit the `build_map_progress` signal.
                // This requires a thread-safe way to access the Godot object,
                // which we'll handle in the `godot_bridge` class.
            }
        }
        godot_print!("Worker thread finished processing tiles.");
    });
    
    // Return the receiver side of the channel to the main thread
    rx
}