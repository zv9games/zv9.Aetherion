//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/builder/threaded.rs

//! 🧵 Spawns the threaded builder pipeline with smart chunk streaming.
//!
//! Converts build options into a noise config, wraps the sync channel in a
//! pacing-aware streamer, and dispatches the map builder thread.

use crate::aetherion::pipeline::data::map_build_options::{MapBuildOptions, GodotNoiseType};
use crate::aetherion::pipeline::builder::builder::spawn_map_builder;
use crate::aetherion::pipeline::builder::streamer::ChunkStreamer;
use crate::godot4::messaging::GodotSync;

/// Initializes and launches the map builder thread with streaming control.
pub fn spawn_builder_thread(sync: GodotSync, options: MapBuildOptions) {
    let config = options.to_noise_config();

    // Wrap sync in a pacing-aware chunk streamer (2ms interval)
    let streamer = ChunkStreamer::new(sync, 2);

    spawn_map_builder(
        streamer,
        config,
        GodotNoiseType::from_str(&options.mode.to_string()).to_internal(),
        options.animate,
        options.black.into(),
        options.blue.into(),
    );
}

//end threaded.rs