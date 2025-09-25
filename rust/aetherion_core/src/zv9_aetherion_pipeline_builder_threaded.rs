//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_pipeline_builder_threaded.rs

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


// 🧵 Spawns the threaded builder pipeline with smart chunk streaming.
//
// Converts build options into a noise config, wraps the sync channel in a
// pacing-aware streamer, and dispatches the map builder thread.

use crate::zv9_prelude::*;
use std::str::FromStr;
use godot::prelude::*;

/// 🧵 Spawns the threaded builder pipeline with smart chunk streaming.
///
/// Converts build options into a noise config, wraps the sync channel in a
/// pacing-aware streamer, and dispatches the map builder thread.
///
/// # Behavior
/// - Runs in a background thread
/// - Uses `ChunkStreamer` to pace tile delivery
/// - Emits map generation request to engine
pub fn spawn_builder_thread(sync: GodotSync, options: MapBuildOptions) {
    let config = options.to_noise_config();

    // Use delivery interval from options (default fallback: 2ms)
    let interval_ms = options.delivery_interval_ms.unwrap_or(2);
    let streamer = ChunkStreamer::new(sync, interval_ms as u64);


    // Attempt to parse noise type from string
    let noise_type = match GodotNoiseType::from_str(&options.mode.to_string()) {
        Ok(nt) => nt.to_internal(),
        Err(_) => {
            godot_error!("Invalid noise type: {}", options.mode);
            return;
        }
    };

    // Log builder dispatch
    godot_print!(
        "🧵 Spawning map builder thread: {}x{}, mode={}, seed={}, interval={}ms",
        options.width,
        options.height,
        options.mode,
        options.seed,
        interval_ms
    );

    // Dispatch builder thread
    spawn_map_builder(
        streamer,
        config,
        noise_type,
        options.animate,
        options.black.into(),
        options.blue.into(),
    );

    // Future: emit signal "builder_thread_started"
}


// the end