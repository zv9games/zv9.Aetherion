
//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_godot_interface_signals.rs

use godot::prelude::*;
#[allow(unused_imports)]
use crate::zv9_prelude::*;

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
		log_component!("AetherionSignals", "Signal node for engine events");
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
    fn chunk_ready(chunk_data: Dictionary);

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
    fn diagnostics(memory_usage: i64, thread_count: i32, tick_rate: f64);

    // ⚠️ Error & warning signals
    #[signal]
    fn rust_error(message: GString);

    #[signal]
    fn rust_warning(message: GString);

    // 🔁 Tilemap & sync events
    #[signal]
    fn tilemap_updated();

    #[signal]
    fn tilemap_sync_complete();

    #[signal]
    fn rust_extension_ready();

    // 🧪 Custom & extensible hooks
    #[signal]
    fn custom_event(name: GString, payload: Variant);
}



// the end