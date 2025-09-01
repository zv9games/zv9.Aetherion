use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};
use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::aetherion::generator::noise::NoiseType;
use crate::aetherion::generator::noise_config::{NoiseConfig, generate_grid_from_config};


use rayon::prelude::*;
use std::time::Instant;

/// Spawns a thread to build a procedural tilemap and stream chunks + signals to Godot.
pub fn spawn_map_builder(
    sync: GodotSync,
    config: NoiseConfig,
    mode: NoiseType,
    animate: bool,
    black: SerializableVector2i,
    blue: SerializableVector2i,
) {
    let grid = generate_grid_from_config(&config, mode);
    let total_tiles = (config.width * config.height) as u64;
    let batch_size = 1_000;

    rayon::spawn(move || {
        let start_time = Instant::now();
        let mut chunk = MapDataChunk::new();
        let mut placed = 0;

        sync.add_signal(EngineMessage::Start);
        sync.add_signal(EngineMessage::Status("Building map".into()));

        for y in 0..config.height as i32 {
            for x in 0..config.width as i32 {
                let pos = SerializableVector2i { x, y };
                let is_black = grid[y as usize][x as usize] == 1;
                let atlas = if is_black { black.clone() } else { blue.clone() };

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
                    sync.add_signal(EngineMessage::Progress(percent));
                    sync.add_chunk(chunk);
                    chunk = MapDataChunk::new();
                }
            }
        }

        if !chunk.is_empty() {

            sync.add_chunk(chunk);
        }

        sync.add_signal(EngineMessage::Progress(100));
        sync.add_signal(EngineMessage::Status("Terrain generation complete".into()));
        sync.add_signal(EngineMessage::Complete {
            width: config.width as i32,
            height: config.height as i32,
            mode: mode.as_str().into(),
            animate,
            duration: start_time.elapsed().as_secs_f64(),
        });
    });
}
