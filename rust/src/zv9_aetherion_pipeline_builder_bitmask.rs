// zv9__aetherion__pipeline_builder__bitmask.rs

use image::open;
use rayon::prelude::*;
use crate::zv9_aetherion_pipeline_data_chunk::MapDataChunk;
use crate::zv9_aetherion_pipeline_data_tile::TileInfo;
use crate::zv9_aetherion_pipeline_data_vector::SerializableVector2i;

/// 🖼️ Converts a PNG image into a scaled tilemap chunk.
/// White pixels become blue tiles, black pixels become black tiles.
pub fn convert_world_png_to_chunk(path: &str, scale: u32) -> MapDataChunk {
    let img = open(path).expect("Failed to load PNG").to_luma8();
    let (w, h) = img.dimensions();
    let scaled_w = w * scale;
    let scaled_h = h * scale;

    let thread_chunks: Vec<MapDataChunk> = (0..scaled_h)
        .into_par_iter()
        .map(|y| {
            let mut local_chunk = MapDataChunk::new();

            for x in 0..scaled_w {
                let src_x = x / scale;
                let src_y = y / scale;
                let pixel = img.get_pixel(src_x, src_y)[0];

                let atlas_coords = if pixel > 128 {
                    SerializableVector2i { x: 1, y: 0 } // Blue tile
                } else {
                    SerializableVector2i { x: 0, y: 0 } // Black tile
                };

                let tile = TileInfo::new(atlas_coords, 0);
                let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                local_chunk.insert(pos, tile);
            }

            local_chunk
        })
        .collect();

    let mut final_chunk = MapDataChunk::new();
    for chunk in thread_chunks {
        final_chunk.merge(chunk);
    }

    final_chunk
}


// the end