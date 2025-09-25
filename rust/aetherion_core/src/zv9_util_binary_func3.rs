//c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary_func3.rs

use crate::pipeline_builder::bitmask::convert_world_png_to_chunk;
use crate::{MapDataChunk, TileInfo};
// use crate::AetherionMap;

use std::time::{Duration, Instant};

// use godot::builtin::{Array, Dictionary, Vector2i};
// use godot::classes::{Node, TileMap};
// use godot::meta::AsArg;
use godot::prelude::*;
use std::collections::HashMap;


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

    let mut tiles = Vec::new();

    for i in 0..100 {
		let mut tile: HashMap<String, Variant> = HashMap::new();

		tile.insert("source_id".to_string(), Variant::from(0));
		tile.insert("atlas_coords".to_string(), Variant::from(format!("{},{}", i % 8, i / 8)));
		tile.insert("alternate_id".to_string(), Variant::from(0));
		tile.insert("rotation".to_string(), Variant::from(0));
		tile.insert("layer".to_string(), Variant::from(0));

		tiles.push(tile);
	}


    println!("✅ Generated {} tiles.", tiles.len());
    println!("📐 Grid preview: 8 x {}", tiles.len() / 8);
}



//the end