
/// ✅ Suggestions for aetherion/godot4/signals.rs

// 🔧 Add signal grouping or categorization:
//     - e.g. `#[signal(group = "generation")]` or prefix conventions
//     - Helps organize signals in GDScript and editor tooling

// 🧩 Add signal documentation:
//     - Inline comments for each signal explaining when and why it fires
//     - Improves clarity for GDScript consumers and plugin authors

// 🚦 Add signal throttling or deduplication:
//     - Prevent flooding with `generation_progress` or `tick_*` signals
//     - Could use internal counters or timestamps

// 📚 Document signal lifecycle:
//     - Clarify which signals fire during init, build, sync, and teardown
//     - Could include a signal flow diagram in module-level docs

// 🧪 Add integration tests for signal dispatch:
//     - Validate that signals are emitted correctly during terrain generation
//     - Ensure payloads match expected formats

// 🧼 Optional: Add signal introspection or logging:
//     - Emit debug logs when signals are fired
//     - Useful for diagnostics and runtime tracing

// 🚀 Future: Add signal subscription API:
//     - e.g. `fn subscribe(signal: &str, callback: Callable)`
//     - Enables dynamic hook registration from GDScript or Rust

// 🧠 Consider exposing signal metadata:
//     - e.g. `fn all_signals() -> Vec<&'static str>`
//     - Useful for editor UIs, debugging, or plugin systems


use godot::prelude::*;

/// 🛰️ AetherionSignals — Godot-facing signal node for engine events.
/// Connected in GDScript to receive updates from the Rust core.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct AetherionSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for AetherionSignals {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("📡 AetherionSignals initialized.");
    }
}

#[godot_api]
impl AetherionSignals {
    // ✅ Core generation signals
    #[signal]
    fn build_map_start();

    #[signal]
    fn generation_progress(percent: i32);

    #[signal]
    fn generation_complete(results: Dictionary);

    #[signal]
    fn map_building_status(status_message: GString);

    #[signal]
    fn map_chunk_ready();

    #[signal]
    fn chunk_ready(chunk_data: Dictionary); // New: full chunk payload

    #[signal]
    fn map_build_cancelled();

    #[signal]
    fn map_build_failed();

    #[signal]
    fn map_build_duration(duration: f64);

    // 🧠 Lifecycle & diagnostics
    #[signal]
    fn tick_started();

    #[signal]
    fn tick_completed();

    #[signal]
    fn frame_budget_exceeded();

    #[signal]
    fn engine_init_start();

    #[signal]
    fn engine_initialized();

    #[signal]
    fn pipeline_start();

    #[signal]
    fn pipeline_complete();

    #[signal]
    fn sync_required();

    #[signal]
    fn diagnostics(memory_usage: i64, thread_count: i32, tick_rate: f64); // New

    // ⚠️ Error & warning signals
    #[signal]
    fn rust_error(message: GString); // Updated to include message

    #[signal]
    fn rust_warning(message: GString); // New

    // 🔁 Tilemap & sync events
    #[signal]
    fn tilemap_updated();

    #[signal]
    fn tilemap_sync_complete();

    #[signal]
    fn rust_extension_ready();

    // 🧪 Custom & extensible hooks
    #[signal]
    fn custom_event(name: GString, payload: Variant); // New
}
