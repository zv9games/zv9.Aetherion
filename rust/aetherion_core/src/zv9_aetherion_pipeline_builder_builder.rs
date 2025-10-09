use crate::zv9_prelude::*;
use crate::zv9_aetherion_generator_noise_config::{generate_grid_from_config, NoiseConfig};
use crate::zv9_aetherion_generator_noise::NoiseType;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};
use crate::pipeline::data::{SerializableVector2i, MapDataChunk, TileInfo};

use rayon::slice::ParallelSlice;
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
        let sync_id = streamer.delivery_mut().sync_id();
        log::info!("🧵 Builder thread launched using Sync[{}]", sync_id);

        // 🚦 Initialization signals
        {
            let sync = streamer.sync();
            let start_signal = EngineMessage::Start;
            let status_signal = EngineMessage::Status("🧬 Building terrain...".into());

            log::info!("📤 Pushing signal: {:?}", start_signal);
            sync.add_signal(start_signal);

            log::info!("📤 Pushing signal: {:?}", status_signal);
            sync.add_signal(status_signal);
        }

        // 🧮 Generate tile positions
        let positions: Vec<SerializableVector2i> = (0..config.height as i32)
            .flat_map(|y| (0..config.width as i32).map(move |x| SerializableVector2i { x, y }))
            .collect();

        // 🧱 Build chunks in parallel
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

        // 🚚 Deliver chunks with progress signals
        for (i, chunk) in chunks.into_iter().enumerate() {
            let percent = (((i + 1) * batch_size).min(total_tiles) * 100 / total_tiles) as i32;
            let progress_signal = EngineMessage::Progress(percent);

            {
                let sync = streamer.sync();
                log::info!("📤 Pushing signal: {:?}", progress_signal);
                sync.add_signal(progress_signal);
            }

            streamer.enqueue_chunk(chunk);
            thread::sleep(Duration::from_millis(2));
        }

        streamer.try_deliver();

        // 🏁 Completion signals
        {
            let sync = streamer.sync();

            let final_progress = EngineMessage::Progress(100);
            let final_status = EngineMessage::Status("✅ Terrain generation complete.".into());
            let complete_signal = EngineMessage::Complete {
                width: config.width as i32,
                height: config.height as i32,
                mode: mode.as_str().to_string(),
                animate,
                duration: start_time.elapsed().as_secs_f64(),
            };

            log::info!("📤 Pushing signal: {:?}", final_progress);
            sync.add_signal(final_progress);

            log::info!("📤 Pushing signal: {:?}", final_status);
            sync.add_signal(final_status);

            log::info!("📤 Pushing signal: {:?}", complete_signal);
            sync.add_signal(complete_signal);
        }
    });
}
