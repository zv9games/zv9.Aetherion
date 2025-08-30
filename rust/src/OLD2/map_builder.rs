use godot::prelude::*;
use godot::classes::TileMap;
use crate::godot_bridge::godotsync::{GodotSync, EngineMessage, TilePlacementMessage, TileInfo};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

pub fn build_map(
    tilemap: Gd<TileMap>,
    width: i32,
    height: i32,
    seed: u64,
    black: Vector2i,
    blue: Vector2i,
    animate: bool,
    mode: String,
    sync: &GodotSync,
) {
    let mut rng = StdRng::seed_from_u64(seed);
    let start = std::time::Instant::now();
    let total_tiles = (width as u64) * (height as u64);
    let batch_size = 10_000;
    let mut tiles = Vec::with_capacity(batch_size as usize);
    let mut placed = 0;

    for y in 0..height {
        for x in 0..width {
            let pos = Vector2i::new(x, y);
            let atlas = match mode.as_str() {
                "checkerboard" => {
                    if (x + y) % 2 == 0 { black } else { blue }
                }
                "clustered" => {
                    if rng.gen_bool(0.7) { black } else { blue }
                }
                "noise" => {
                    if rng.gen_bool(0.5) { black } else { blue }
                }
                _ => {
                    godot_warn!("⚠️ Unknown mode '{}', defaulting to noise.", mode);
                    if rng.gen_bool(0.5) { black } else { blue }
                }
            };

            tiles.push(TilePlacementMessage {
                position: pos,
                info: TileInfo {
                    source_id: 0,
                    atlas_coords: atlas,
                    alternate_id: 0,
                },
            });

            placed += 1;
            if placed % batch_size == 0 || placed == total_tiles {
                let percent = ((placed * 100) / total_tiles) as i32;
                sync.add_signal(EngineMessage::Progress(percent));
                sync.add_tiles(tiles);
                godot_print!("📊 Generated {} tiles, {}%", placed, percent);
                tiles = Vec::with_capacity(batch_size as usize);
                // Yield to Godot
                if let Some(mut tree) = tilemap.get_tree() {
                    tree.call("process_frame", &[]);
                }
                godot_print!("🛠️ Yielded to Godot main thread");
            }
        }
    }

    if !tiles.is_empty() {
        let remaining = tiles.len();
        sync.add_tiles(tiles);
        godot_print!("🛠️ Added final batch of {} tiles", remaining);
    }

    godot_print!("🛠️ Ensuring tile sync");
    let duration = start.elapsed().as_secs_f64();
    sync.add_signal(EngineMessage::Status("Terrain generation complete".to_string()));
    sync.add_signal(EngineMessage::Complete {
        width,
        height,
        mode,
        animate,
        duration,
    });
    godot_print!("✅ Generation finished: {} tiles in {:.2}s", placed, duration);
}