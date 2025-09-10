use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};
use crate::godot4::messaging::EngineMessage;
use crate::aetherion::generator::noise::{NoiseType};
use crate::aetherion::generator::noise_config::{NoiseConfig, generate_grid_from_config};
use crate::aetherion::pipeline::builder::streamer::ChunkStreamer;

use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::thread;

pub fn spawn_map_builder(
    mut streamer: ChunkStreamer,
    config: NoiseConfig,
    mode: NoiseType,
    animate: bool,
    black: SerializableVector2i,
    blue: SerializableVector2i,
) {
    let grid = generate_grid_from_config(&config, mode);
    let total_tiles = (config.width * config.height) as usize;
    let batch_size = ((total_tiles / 100).max(500).min(10_000)) as usize;

    rayon::spawn(move || {
        let start_time = Instant::now();
        streamer.sync().add_signal(EngineMessage::Start);
        streamer.sync().add_signal(EngineMessage::Status("🧬 Building terrain...".to_string()));

        // Flatten grid into tile positions
        let positions: Vec<SerializableVector2i> = (0..config.height as i32)
            .flat_map(|y| (0..config.width as i32).map(move |x| SerializableVector2i { x, y }))
            .collect();

        // Parallel chunk construction
        let chunks: Vec<(usize, MapDataChunk)> = positions
            .par_chunks(batch_size)
            .enumerate()
            .map(|(i, batch)| {
                let mut chunk = MapDataChunk::new();

                for pos in batch {
                    let is_land = grid[pos.y as usize][pos.x as usize] == 1;
                    let atlas = if is_land { black.clone() } else { blue.clone() };

                    chunk.insert(pos.clone(), TileInfo {
                        source_id: 0,
                        atlas_coords: atlas,
                        alternate_id: 0,
                        flags: 0,
                        layer: 0,
                        rotation: 0,
                    });
                }

                (i, chunk)
            })
            .collect();

        // Sequential delivery with pacing
        for (i, chunk) in chunks {
            let percent = (((i + 1) * batch_size).min(total_tiles) * 100 / total_tiles) as i32;
            streamer.sync().add_signal(EngineMessage::Progress(percent));
            streamer.enqueue_chunk(chunk);
            thread::sleep(Duration::from_millis(2));
        }

        streamer.try_deliver();
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
