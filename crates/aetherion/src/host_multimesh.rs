//! Plan-B-lite host path: flood a `MultiMeshInstance2D` with colored quads.
//!
//! Quads are **16×16 world units** (readable on screen). Godot requires
//! `instance_count == 0` before changing format/colors, so each apply uses a fresh MultiMesh.

use crate::chunk::ChunkData;
use godot::classes::multi_mesh::TransformFormat;
use godot::classes::{MultiMesh, MultiMeshInstance2D, QuadMesh};
use godot::prelude::*;
use std::time::Instant;

/// World size of one tile quad (matches demo TileMap-ish scale).
pub const TILE_WORLD_SIZE: f32 = 16.0;

fn color_for_tile(tile: u16) -> Color {
    match tile {
        0 | 1 => Color::from_rgb(0.12, 0.85, 0.40),
        2 => Color::from_rgb(0.25, 0.45, 1.0),
        3 => Color::from_rgb(1.0, 0.75, 0.15),
        _ => Color::from_rgb(0.95, 0.30, 0.65),
    }
}

fn new_colored_multimesh(count: i32) -> Gd<MultiMesh> {
    let mut mm = MultiMesh::new_gd();
    let mut quad = QuadMesh::new_gd();
    // Unit quad; we scale per-instance via transform basis.
    quad.set_size(Vector2::new(1.0, 1.0));
    mm.set_mesh(&quad);
    mm.set_instance_count(0);
    mm.set_transform_format(TransformFormat::TRANSFORM_2D);
    mm.set_use_colors(true);
    mm.set_instance_count(count.max(0));
    mm
}

/// Apply all chunk tiles as MultiMesh instances.
/// Returns (instances, elapsed_ms).
pub fn apply_chunks_to_multimesh(
    mmi: &mut Gd<MultiMeshInstance2D>,
    chunks: &[ChunkData],
) -> (u64, u128) {
    let total: i32 = chunks
        .iter()
        .map(|c| c.tile_count() as i32)
        .sum::<i32>()
        .max(0);

    let t0 = Instant::now();
    let mut mm = new_colored_multimesh(total);
    let s = TILE_WORLD_SIZE;

    let mut i = 0i32;
    for chunk in chunks {
        let origin_x = chunk.coord.x * chunk.size as i32;
        let origin_y = chunk.coord.y * chunk.size as i32;
        let size = chunk.size;
        for ly in 0..size {
            let row = (ly * size) as usize;
            for lx in 0..size {
                let tile = chunk.tiles[row + lx as usize];
                let cx = (origin_x + lx as i32) as f32 * s + s * 0.5;
                let cy = (origin_y + ly as i32) as f32 * s + s * 0.5;
                // Scale unit quad to TILE_WORLD_SIZE and place at cell center.
                let gap = 0.92;
                let xf = Transform2D {
                    a: Vector2::new(s * gap, 0.0),
                    b: Vector2::new(0.0, s * gap),
                    origin: Vector2::new(cx, cy),
                };
                mm.set_instance_transform_2d(i, xf);
                mm.set_instance_color(i, color_for_tile(tile));
                i += 1;
            }
        }
    }
    mmi.set_multimesh(&mm);
    (total as u64, t0.elapsed().as_millis())
}
