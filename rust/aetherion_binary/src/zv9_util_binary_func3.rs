use crate::pipeline::data::{MapDataChunk, TileInfo};
use crate::pipeline::builder::convert_world_png_to_chunk;


use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 🧪 Benchmarks tile placement throughput over 30 seconds
pub fn run_max_grid_benchmark() {
    println!("🧪 Starting max grid benchmark (30s)…");

    let mut chunk = MapDataChunk::default();
    let start = Instant::now();
    let time_limit = Duration::from_secs(30);
    let mut tiles_placed = 0;
    let mut last_logged = Instant::now();

    let grid_width = 10_000;
    let mut x = 0;
    let mut y = 0;

    while Instant::now() - start < time_limit {
        chunk.place_tile(x, y, TileInfo::default());
        tiles_placed += 1;

        x += 1;
        if x >= grid_width {
            x = 0;
            y += 1;
        }

        if Instant::now() - last_logged >= Duration::from_secs(1) {
            println!(
                "⏱ {}s elapsed — {} tiles placed",
                (Instant::now() - start).as_secs(),
                tiles_placed
            );
            last_logged = Instant::now();
        }
    }

    println!("\n✅ Benchmark complete.");
    println!("🧱 Total tiles placed: {}", tiles_placed);
    println!("📐 Final grid size: {} x {}", grid_width, y + 1);
    println!("⚡ Throughput: ~{} tiles/sec", tiles_placed / 30);
}

/// 🧪 Converts a PNG into a tile chunk using bitmask logic
pub fn run_bitmask_conversion() {
    println!("🧪 Starting bitmask conversion from world.png…");

    let path = "C:/ZV9/zv9.aetherion/.assets/world.png";
    let scale = 3;
    let start = Instant::now();

    let chunk = convert_world_png_to_chunk(path, scale);
    let elapsed = start.elapsed();

    println!("✅ Conversion complete.");
    println!("🧱 Tiles placed: {}", chunk.len());
    println!(
        "📐 Final grid size: ~{} x {}",
        (chunk.len() as f64).sqrt().round() as u32,
        (chunk.len() as f64).sqrt().round() as u32
    );
    println!("⏱ Time taken: {:.2?}", elapsed);
}

/// 🧪 CLI-safe test: generates and prints a test chunk
pub fn test_generation_and_placement_cli() {
    println!("🧪 Running CLI test for generation and placement...");

    #[derive(Debug)]
    struct Tile {
        source_id: i32,
        atlas_coords: (i32, i32),
        alternate_id: i32,
        rotation: i32,
        layer: i32,
    }

    let mut tiles = Vec::new();

    for i in 0..100 {
        let tile = Tile {
            source_id: 0,
            atlas_coords: (i % 8, i / 8),
            alternate_id: 0,
            rotation: 0,
            layer: 0,
        };
        tiles.push(tile);
    }

    println!("✅ Generated {} tiles.", tiles.len());
    println!("📐 Grid preview: 8 x {}", tiles.len() / 8);
}
