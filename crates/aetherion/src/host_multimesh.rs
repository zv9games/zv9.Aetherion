//! Plan-B-lite host path: flood a `MultiMeshInstance2D` with colored quads.
//!
//! Much faster than per-cell TileMap writes for large demos (SSXL-ext mesh lineage, 2D).

use crate::chunk::ChunkData;
use godot::classes::multi_mesh::TransformFormat;
use godot::classes::{MultiMesh, MultiMeshInstance2D, QuadMesh};
use godot::prelude::*;
use std::time::Instant;

fn color_for_tile(tile: u16) -> Color {
    match tile {
        0 | 1 => Color::from_rgb(0.15, 0.7, 0.35),
        2 => Color::from_rgb(0.2, 0.4, 0.9),
        3 => Color::from_rgb(0.9, 0.65, 0.15),
        _ => Color::from_rgb(0.75, 0.25, 0.55),
    }
}

/// Ensure instance has a colored MultiMesh with a unit quad.
fn ensure_multimesh(mmi: &mut Gd<MultiMeshInstance2D>, count: i32) -> Gd<MultiMesh> {
    let mut mm = mmi.get_multimesh().unwrap_or_else(|| {
        let mut m = MultiMesh::new_gd();
        let mut quad = QuadMesh::new_gd();
        quad.set_size(Vector2::new(1.0, 1.0));
        m.set_mesh(&quad);
        m.set_transform_format(TransformFormat::TRANSFORM_2D);
        m.set_use_colors(true);
        mmi.set_multimesh(&m);
        m
    });

    // Reconfigure mesh/format if needed
    if mm.get_mesh().is_none() {
        let mut quad = QuadMesh::new_gd();
        quad.set_size(Vector2::new(1.0, 1.0));
        mm.set_mesh(&quad);
    }
    mm.set_transform_format(TransformFormat::TRANSFORM_2D);
    mm.set_use_colors(true);
    if mm.get_instance_count() != count {
        mm.set_instance_count(count);
    }
    mmi.set_multimesh(&mm);
    mm
}

/// Apply all chunk tiles as MultiMesh instances (1 unit = 1 tile).
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
    let mut mm = ensure_multimesh(mmi, total);

    let mut i = 0i32;
    for chunk in chunks {
        let origin_x = chunk.coord.x * chunk.size as i32;
        let origin_y = chunk.coord.y * chunk.size as i32;
        let size = chunk.size;
        for ly in 0..size {
            let row = (ly * size) as usize;
            for lx in 0..size {
                let tile = chunk.tiles[row + lx as usize];
                let x = (origin_x + lx as i32) as f32;
                let y = (origin_y + ly as i32) as f32;
                // Center quad on cell
                let xf = Transform2D::from_angle_origin(0.0, Vector2::new(x + 0.5, y + 0.5));
                mm.set_instance_transform_2d(i, xf);
                mm.set_instance_color(i, color_for_tile(tile));
                i += 1;
            }
        }
    }
    // Write back in case Godot needs the resource reassigned
    mmi.set_multimesh(&mm);
    (total as u64, t0.elapsed().as_millis())
}
