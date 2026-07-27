//! Apply generated chunks to a Godot `TileMap` (SSXL-ext host_tilemap lineage).
//!
//! Builds a 4-color procedural atlas when the map has no TileSet, then batches `set_cell_ex`.

use crate::chunk::ChunkData;
use godot::classes::image::Format;
use godot::classes::{Image, ImageTexture, TileMap, TileSet, TileSetAtlasSource};
use godot::prelude::*;
use std::time::Instant;

/// Ensure `tilemap` has a simple atlas source with 4 colored 16×16 tiles.
pub fn ensure_demo_tileset(tilemap: &mut Gd<TileMap>) {
    if tilemap.get_tileset().is_some() {
        return;
    }

    let mut image = Image::create(64, 16, false, Format::RGBA8).expect("create image");
    let colors: [(u8, u8, u8, u8); 4] = [
        (30, 30, 40, 255),
        (40, 180, 90, 255),
        (50, 100, 220, 255),
        (220, 160, 40, 255),
    ];
    for (ti, (r, g, b, a)) in colors.iter().enumerate() {
        let x0 = (ti as i32) * 16;
        for y in 0..16 {
            for x in 0..16 {
                let edge = x == 0 || y == 0 || x == 15 || y == 15;
                let (cr, cg, cb) = if edge {
                    (
                        r.saturating_add(40),
                        g.saturating_add(40),
                        b.saturating_add(40),
                    )
                } else {
                    (*r, *g, *b)
                };
                image.set_pixel(x0 + x, y, Color::from_rgba8(cr, cg, cb, *a));
            }
        }
    }

    let tex = ImageTexture::create_from_image(&image).expect("texture");
    let mut atlas = TileSetAtlasSource::new_gd();
    atlas.set_texture(&tex);
    atlas.set_texture_region_size(Vector2i::new(16, 16));
    for i in 0..4 {
        let coords = Vector2i::new(i, 0);
        if !atlas.has_tile(coords) {
            atlas.create_tile(coords);
        }
    }

    let mut tileset = TileSet::new_gd();
    let _source_id = tileset.add_source(&atlas);
    tilemap.set_tileset(&tileset);
    godot_print!("[Aetherion] installed procedural demo TileSet (4 atlas tiles)");
}

#[inline]
fn atlas_x_for_tile(tile: u16) -> i32 {
    match tile {
        0 | 1 => 0,
        2 => 1,
        3 => 2,
        _ => 3,
    }
}

/// Apply chunks onto layer 0 of `tilemap`. Returns cells written + elapsed ms.
///
/// Clears the layer once, writes all cells, then calls `update_internals`.
pub fn apply_chunks_to_tilemap(tilemap: &mut Gd<TileMap>, chunks: &[ChunkData]) -> (u64, u128) {
    ensure_demo_tileset(tilemap);
    let t0 = Instant::now();
    tilemap.clear_layer(0);

    let mut cells: u64 = 0;
    let source_id = 0;

    for chunk in chunks {
        let origin_x = chunk.coord.x * chunk.size as i32;
        let origin_y = chunk.coord.y * chunk.size as i32;
        let size = chunk.size;
        let tiles = &chunk.tiles;
        for ly in 0..size {
            let row = (ly * size) as usize;
            let map_y = origin_y + ly as i32;
            for lx in 0..size {
                let tile = tiles[row + lx as usize];
                let map_x = origin_x + lx as i32;
                tilemap
                    .set_cell_ex(0, Vector2i::new(map_x, map_y))
                    .source_id(source_id)
                    .atlas_coords(Vector2i::new(atlas_x_for_tile(tile), 0))
                    .done();
                cells += 1;
            }
        }
    }
    tilemap.update_internals();
    (cells, t0.elapsed().as_millis())
}
