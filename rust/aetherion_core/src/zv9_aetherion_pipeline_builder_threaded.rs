#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use std::str::FromStr;
use crate::structure::MapBuildOptions;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery, spawn_map_builder};
use crate::zv9_aetherion_generator_noise::NoiseType;

/// 🧵 Spawns a threaded map builder using delivery backend and build options.
pub fn spawn_builder_thread(
    delivery: impl ChunkDelivery + Send + 'static,
    options: MapBuildOptions,
) {
    let config = options.to_noise_config();
    let interval_ms = options.delivery_interval_ms.unwrap_or(2);
    let mut streamer = ChunkStreamer::new(delivery, interval_ms as u64);

    let noise_type = match NoiseType::from_str(&options.mode.to_string()) {
        Ok(nt) => nt,
        Err(e) => {
            println!("❌ NoiseType parse error: {:?}", e);
            return;
        }
    };

    println!(
        "🧵 Spawning builder thread: {}x{}, mode={}, seed={}, interval={}ms",
        options.width,
        options.height,
        options.mode,
        options.seed,
        interval_ms
    );

    spawn_map_builder(
        &mut streamer,
        config,
        noise_type,
        options.animate,
        options.black.into(),
        options.blue.into(),
    );
}
