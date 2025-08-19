use std::thread;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use godot::prelude::*;
use rand::{Rng, SeedableRng}; // ✅ Rng now used via gen/gen_bool
use rand::rngs::StdRng;
use crate::types::{TilePayload, WALL_TILE, FLOOR_TILE, ZERO_TILE, ONE_TILE};

/// Shared queue for tile glyphs prepared off-thread
pub static TILE_QUEUE: Lazy<Arc<Mutex<Vec<TilePayload>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

/// Struct for changeover updates
#[derive(Clone)]
pub struct ChangeoverUpdate {
    pub source_id: i32,
    pub tiles: Vec<(Vector2i, Vector2i, i32)>, // (pos, atlas_coords, alt_id)
    pub log_messages: Vec<String>, // Queued log messages
}

/// Shared queue for changeover updates
pub static CHANGEOVER_QUEUE: Lazy<Arc<Mutex<Vec<ChangeoverUpdate>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[allow(dead_code)] // Called from GDScript
/// Async tile decoder — runs in its own thread, pushes into TILE_QUEUE
pub fn generate_payload_async(image_data: Vec<u8>, width: i32, height: i32, chunk_size: i32) {
    thread::spawn(move || {
        let mut log_messages = Vec::new();
        if image_data.len() < (width * height * 4) as usize {
            log_messages.push(format!(
                "Invalid image data size: {} for {}x{}", 
                image_data.len(), width, height
            ));
            return;
        }

        let chunks: Vec<(i32, i32)> = (0..height)
            .step_by(chunk_size as usize)
            .flat_map(|y| (0..width).step_by(chunk_size as usize).map(move |x| (x, y)))
            .collect();

        let chunk_results: Vec<(Vec<TilePayload>, Vec<String>)> = chunks.par_iter().map(|&(start_x, start_y)| {
            let mut glyphs = Vec::new();
            let mut chunk_logs = Vec::new();
            let end_x = (start_x + chunk_size).min(width);
            let end_y = (start_y + chunk_size).min(height);

            for y in start_y..end_y {
                for x in start_x..end_x {
                    let lum = compute_luminance(&image_data, x, y, width, &mut chunk_logs);
                    let label = match lum {
                        l if l > 0.8 => "wall",
                        l if l < 0.2 => "floor",
                        _ => continue,
                    };

                    let payload = TilePayload {
                        pos: Vector2i::new(x - width / 2, y - height / 2),
                        source_id: 0,
                        atlas_coords: if label == "wall" { WALL_TILE } else { FLOOR_TILE },
                        alt_id: 0,
                        label: Some(label.to_string()),
                    };

                    glyphs.push(payload);
                }
            }
            (glyphs, chunk_logs)
        }).collect();

        let mut queue = TILE_QUEUE.lock().unwrap();
        for (glyphs, chunk_logs) in chunk_results {
            queue.extend(glyphs);
            log_messages.extend(chunk_logs);
        }
        log_messages.push(format!(
            "🛠️ Generated {} tiles in async thread.", 
            queue.len()
        ));

        // Push log messages to CHANGEOVER_QUEUE
        let update = ChangeoverUpdate {
            source_id: 0,
            tiles: Vec::new(),
            log_messages,
        };
        let mut changeover_queue = CHANGEOVER_QUEUE.lock().unwrap();
        changeover_queue.push(update);
    });
}

/// Generate changeover updates for a frame
pub fn generate_changeover_async(
    width: i32,
    height: i32,
    source_id: i32,
    frame: i32,
    seed: i64,
    flip_rate: f32,
) {
    thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64((seed as u64) ^ (frame as u64));
        let mut tiles = Vec::new();
        let mut log_messages = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if rng.gen::<f32>() < flip_rate {
                    let pos = Vector2i::new(x, y);
                    let glyph = if rng.gen_bool(0.5) { ZERO_TILE } else { ONE_TILE };
                    tiles.push((pos, glyph, 0));
                }
            }
        }

        log_messages.push(format!(
            "🎬 Generated changeover frame {} with {} updates.", 
            frame, tiles.len()
        ));

        let update = ChangeoverUpdate {
            source_id,
            tiles,
            log_messages,
        };

        let mut queue = CHANGEOVER_QUEUE.lock().unwrap();
        queue.push(update);
    });
}

#[allow(dead_code)] // Used by generate_payload_async
/// Computes luminance from an RGBA byte array at (x, y)
fn compute_luminance(image_data: &[u8], x: i32, y: i32, width: i32, log_messages: &mut Vec<String>) -> f32 {
    let idx = ((y * width + x) * 4) as usize;
    if idx + 3 >= image_data.len() {
        log_messages.push(format!(
            "Invalid image data index: {} for ({}, {})", 
            idx, x, y
        ));
        return 0.0;
    }

    let r = image_data[idx] as f32 / 255.0;
    let g = image_data[idx + 1] as f32 / 255.0;
    let b = image_data[idx + 2] as f32 / 255.0;
    let a = image_data[idx + 3] as f32 / 255.0;

    if a < 0.5 {
        0.0
    } else {
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
}

/// Pulls everything currently in the tile queue, emptying it
pub fn take_payload_batch() -> Vec<TilePayload> {
    let mut q = TILE_QUEUE.lock().unwrap();
    q.drain(..).collect()
}

/// Pulls everything currently in the changeover queue, emptying it
pub fn take_changeover_batch() -> Vec<ChangeoverUpdate> {
    let mut q = CHANGEOVER_QUEUE.lock().unwrap();
    q.drain(..).collect()
}
