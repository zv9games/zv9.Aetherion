//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/builder/threaded.rs

/// ✅ Suggestions for aetherion/pipeline/builder/threaded.rs

// 🔧 Make delivery interval configurable:
//     - Replace hardcoded `2ms` with `options.delivery_interval_ms`
//     - Enables tuning for performance vs responsiveness

// 🧩 Add thread lifecycle tracking:
//     - Emit signals for thread start, completion, or failure
//     - Useful for diagnostics and editor feedback

// 🚦 Add error handling for builder dispatch:
//     - Wrap `spawn_map_builder()` in a `Result`
//     - Emit failure signal if config or mode is invalid

// 📚 Document threading behavior and safety:
//     - Clarify that builder runs in background thread
//     - Note that `ChunkStreamer` handles pacing to avoid frame spikes

// 🧪 Add unit tests or mocks for thread dispatch:
//     - Validate correct config conversion and builder invocation
//     - Ensure sync channel receives expected signals

// 🧼 Optional: Add logging or debug output:
//     - Log thread spawn time, config summary, and mode
//     - Useful for profiling and runtime introspection

// 🚀 Future: Support multiple concurrent builders:
//     - e.g. terrain + overlays + modifiers in parallel
//     - Could expose a `BuilderPool` or task manager

// 🧠 Consider exposing builder presets:
//     - e.g. “cave”, “island”, “plains” with preconfigured `MapBuildOptions`
//     - Useful for quick prototyping or editor dropdowns


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