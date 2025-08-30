use godot::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use crate::data_processing::types::{MapDataChunk, TileInfo};

/// Message sent from worker threads to the main thread
#[derive(Clone, Debug)]
pub enum EngineMessage {
    /// Place a tile at a specific position
    Tile(TilePlacementMessage),
    /// Update progress percentage
    Progress(i32),
    /// Signal that map generation is complete
    Complete {
        width: i32,
        height: i32,
        mode: String,
        animate: bool,
        duration: f64,
    },
}

/// Tile placement payload
#[derive(Clone, Debug)]
pub struct TilePlacementMessage {
    pub position: Vector2i,
    pub info: TileInfo,
}

/// Spawns a worker thread to stream tile placement messages to the main thread
pub fn spawn_tile_placement_task(
    tile_data: MapDataChunk,
    width: i32,
    height: i32,
    mode: String,
    animate: bool,
) -> Receiver<EngineMessage> {
    let (tx, rx) = crossbeam_channel::unbounded::<EngineMessage>();

    std::thread::spawn(move || {
        godot_print!("🧵 Tile placement thread started.");
        godot_print!("📐 Grid size: {}x{} ({} tiles)", width, height, tile_data.len());

        let total_tiles = tile_data.len();
        let mut placed_count = 0;
        let start = std::time::Instant::now();

        for (pos, info) in tile_data.iter() {
            let tile_msg = TilePlacementMessage {
                position: pos.clone().into(),
                info: info.clone(),
            };

            if tx.send(EngineMessage::Tile(tile_msg)).is_err() {
                godot_error!("❌ Tile message failed — receiver dropped.");
                return;
            }

            placed_count += 1;

            if placed_count % 10_000 == 0 {
                let percent = ((placed_count * 100) / total_tiles) as i32;
                let _ = tx.send(EngineMessage::Progress(percent));
            }
        }

        let duration = start.elapsed().as_secs_f64();

        let completion_msg = EngineMessage::Complete {
            width,
            height,
            mode,
            animate,
            duration,
        };

        if tx.send(completion_msg).is_err() {
            godot_error!("❌ Completion message failed — receiver dropped.");
        }

        godot_print!("✅ Finished placing {} tiles in {:.2} seconds.", placed_count, duration);
    });

    rx
}
