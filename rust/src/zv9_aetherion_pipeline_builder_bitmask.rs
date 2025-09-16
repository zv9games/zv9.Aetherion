// zv9__aetherion__pipeline_builder__bitmask.rs

pub fn convert_png_to_tilemap(path: &str, seed: u64) -> MapDataChunk {
    use image::open;
    use rayon::prelude::*;
    let img = open(path).expect("Failed to load PNG").to_luma8();
    let (width, height) = img.dimensions();

    let mut chunk = MapDataChunk::new();

    (0..height).into_par_iter().for_each(|y| {
        for x in 0..width {
            let pixel = img.get_pixel(x, y)[0];
            let tile = bitmask_to_tile(pixel, x, y, seed);
            let pos = SerializableVector2i { x: x as i32, y: y as i32 };
            chunk.insert(pos, tile);
        }
    });

    chunk
}

// the end