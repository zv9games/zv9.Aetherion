//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/builder/builder.rs

//! 🧱 Spawns a thread to build a procedural tilemap and stream chunks + signals to Godot.
//!
//! Converts a noise-generated grid into tile chunks, batches them for delivery
//! through a pacing-aware streamer, and emits progress/status signals.

use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};
use crate::godot4::messaging::EngineMessage;
use crate::aetherion::generator::noise::{NoiseType};
use crate::aetherion::generator::noise_config::{NoiseConfig, generate_grid_from_config};
use crate::aetherion::pipeline::builder::streamer::ChunkStreamer;

use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::thread;

/// Launches a threaded map builder that streams chunked tile data to Godot.
pub fn spawn_map_builder(
    mut streamer: ChunkStreamer,
    config: NoiseConfig,
    mode: NoiseType,
    animate: bool,
    black: SerializableVector2i,
    blue: SerializableVector2i,
) {
    let grid = generate_grid_from_config(&config, mode);
    let total_tiles = (config.width * config.height) as u64;
    let batch_size = ((total_tiles / 100).max(500).min(10_000)) as usize;

    rayon::spawn(move || {
        let start_time = Instant::now();
        let mut chunk = MapDataChunk::new();
        let mut placed = 0;

        streamer.sync().add_signal(EngineMessage::Start);
        streamer.sync().add_signal(EngineMessage::Status("🧬 Building terrain...".to_string()));

        for y in 0..config.height as i32 {
            for x in 0..config.width as i32 {
                let pos = SerializableVector2i { x, y };
                let is_land = grid[y as usize][x as usize] == 1;
                let atlas = if is_land { black.clone() } else { blue.clone() };

                chunk.insert(pos, TileInfo {
                    source_id: 0,
                    atlas_coords: atlas,
                    alternate_id: 0,
                    flags: 0,
                    layer: 0,
                    rotation: 0,
                });

                placed += 1;

                if placed % batch_size == 0 || placed as u64 == total_tiles {
                    let percent = ((placed as u64 * 100) / total_tiles) as i32;
                    streamer.sync().add_signal(EngineMessage::Progress(percent));
                    streamer.enqueue_chunk(chunk);
                    //streamer.try_deliver();
                    chunk = MapDataChunk::new();

                    // Allow Godot's main thread to breathe
                    thread::sleep(Duration::from_millis(2));
                }
            }
        }

        // Final flush if any tiles remain
        if !chunk.is_empty() {
            streamer.enqueue_chunk(chunk);
            streamer.try_deliver();
        }

        streamer.sync().add_signal(EngineMessage::Progress(100));
        streamer.sync().add_signal(EngineMessage::Status("✅ Terrain generation complete.".to_string()));
        streamer.sync().add_signal(EngineMessage::Complete {
            width: config.width as i32,
            height: config.height as i32,
            mode: mode.as_str().into(),
            animate,
            duration: start_time.elapsed().as_secs_f64(),
        });
    });
}

//end builder.rs