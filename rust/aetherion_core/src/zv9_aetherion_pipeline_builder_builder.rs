use crate::zv9_prelude::*;
use crate::zv9_aetherion_generator_noise_config::generate_grid_from_config;
use crate::zv9_aetherion_generator_noise_config::NoiseConfig;
use crate::zv9_aetherion_generator_noise::NoiseType;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};
use crate::pipeline::data::{SerializableVector2i, MapDataChunk, TileInfo};

use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::thread;

/// 🚀 Spawns a threaded terrain builder using noise configuration and delivery pacing.
pub fn spawn_map_builder<D: ChunkDelivery + Send + 'static>(
    mut streamer: ChunkStreamer<D>,
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

        {
            let sync = streamer.sync();
            sync.add_signal(EngineMessage::Start);
            sync.add_signal(EngineMessage::Status("🧬 Building terrain...".to_string()));
        }

        let positions: Vec<SerializableVector2i> = (0..config.height as i32)
            .flat_map(|y| (0..config.width as i32).map(move |x| SerializableVector2i { x, y }))
            .collect();

        let chunks: Vec<MapDataChunk> = positions
            .par_chunks(batch_size)
            .map(|batch| {
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
                        variant_id: None,
                        frame_count: None,
                        animation_speed: None,
                    });
                }

                chunk
            })
            .collect();

        for (i, chunk) in chunks.into_iter().enumerate() {
            let percent = (((i + 1) * batch_size).min(total_tiles) * 100 / total_tiles) as i32;

            {
                let sync = streamer.sync();
                sync.add_signal(EngineMessage::Progress(percent));
            }

            streamer.enqueue_chunk(chunk);
            thread::sleep(Duration::from_millis(2));
        }

        streamer.try_deliver();

        {
            let sync = streamer.sync();
            sync.add_signal(EngineMessage::Progress(100));
            sync.add_signal(EngineMessage::Status("✅ Terrain generation complete.".to_string()));
            sync.add_signal(EngineMessage::Complete {
                width: config.width as i32,
                height: config.height as i32,
                mode: mode.as_str().to_string(),
                animate,
                duration: start_time.elapsed().as_secs_f64(),
            });
        }
    });
}
