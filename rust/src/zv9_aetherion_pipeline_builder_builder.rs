//c:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_pipeline_builder_builder.rs

/// ✅ Suggestions for spawn_map_builder module

// 🔧 Add error handling for grid generation:
//     - `generate_grid_from_config()` could fail or return malformed data
//     - Consider wrapping in `Result` and emitting failure signals

// 🧩 Make tile styling configurable:
//     - Instead of hardcoded `black` and `blue`, use a `TileStyle` or `ThemeConfig`
//     - Enables biome-aware or user-defined visuals

// 🚦 Add cancellation or timeout support:
//     - Long terrain builds could be aborted mid-process
//     - Consider exposing a cancel flag or timeout duration

// 📚 Document threading behavior:
//     - Clarify that terrain generation runs in a background thread
//     - Note that `rayon::spawn` is non-blocking and safe for UI contexts

// 🧪 Add unit tests for chunk batching and tile assignment:
//     - Validate correct land/water mapping and chunk sizes
//     - Ensure progress signals match expected percentages

// 🧼 Optional: Add progress smoothing or animation pacing:
//     - Replace fixed `2ms` sleep with adaptive pacing based on system load
//     - Could integrate with `RuntimeState` or diagnostics

// 🚀 Future: Add support for multi-layer terrain or overlays:
//     - e.g. terrain + vegetation + structures
//     - Could extend `TileInfo` or chunk composition logic

// 🧠 Consider exposing build presets or profiles:
//     - e.g. “island”, “cave”, “plains” with preconfigured `NoiseConfig` and tile mappings
//     - Useful for editor integration or quick prototyping


use crate::zv9_prelude::*;
use crate::zv9_aetherion_generator_noise_config::generate_grid_from_config;



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
						variant_id: None,
						frame_count: None,
						animation_speed: None,
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


// the end