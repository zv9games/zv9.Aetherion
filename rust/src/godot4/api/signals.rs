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
    fn rust_error();

    // 🔁 Tilemap & map events
    #[signal]
    fn map_chunk_ready();

    #[signal]
    fn tilemap_updated();

    #[signal]
    fn tilemap_sync_complete();

    #[signal]
    fn map_build_cancelled();

    #[signal]
    fn map_build_failed();

    #[signal]
    fn map_build_duration(duration: f64);

    #[signal]
    fn rust_extension_ready();
}
