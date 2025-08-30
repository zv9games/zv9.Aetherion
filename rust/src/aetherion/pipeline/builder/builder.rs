use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};
use crate::godot4::messaging::{GodotSync, EngineMessage};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

/// Spawns a thread to build a procedural tilemap and stream chunks + signals to Godot.
pub fn spawn_map_builder(
    sync: GodotSync,
    width: i32,
    height: i32,
    seed: u64,
    mode: String, // ✅ Owned string
    animate: bool,
    black: SerializableVector2i,
    blue: SerializableVector2i,
) {
    rayon::spawn(move || {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let total_tiles = (width as u64) * (height as u64);
        let batch_size = 1_000;
        let mut chunk = MapDataChunk::new();
        let mut placed = 0;

        sync.add_signal(EngineMessage::Start);
        sync.add_signal(EngineMessage::Status("Building map".into()));

        for y in 0..height {
            for x in 0..width {
                let pos = SerializableVector2i { x, y };
                let atlas = match mode.as_str() {
                    "checkerboard" => if (x + y) % 2 == 0 { black.clone() } else { blue.clone() },
                    "clustered" => if rng.gen_bool(0.7) { black.clone() } else { blue.clone() },
                    "noise" => if rng.gen_bool(0.5) { black.clone() } else { blue.clone() },
                    _ => if rng.gen_bool(0.5) { black.clone() } else { blue.clone() },
                };

                chunk.insert(pos, TileInfo {
                    source_id: 0,
                    atlas_coords: atlas,
                    alternate_id: 0,
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

        if !chunk.chunk.is_empty() {
            sync.add_chunk(chunk);
        }

        sync.add_signal(EngineMessage::Progress(100));
        let duration = std::time::Instant::now().elapsed().as_secs_f64();
        sync.add_signal(EngineMessage::Status("Terrain generation complete".into()));
        sync.add_signal(EngineMessage::Complete {
            width,
            height,
            mode,
            animate,
            duration,
        });
    });
}
