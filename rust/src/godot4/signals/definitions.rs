use godot::prelude::*;

/// Godot-facing signal node for emitting engine events.
/// This node is exported and connected in GDScript to receive updates.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionSignals {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl AetherionSignals {
    fn ready(&mut self) {
        godot_print!("📡 AetherionSignals is ready.");
    }

    // ✅ Core signals
    #[signal]
    fn build_map_start();

    #[signal]
    fn generation_progress(percent: i32);

    #[signal]
    fn generation_complete(results: Dictionary);

    #[signal]
    fn map_building_status(status: GString);

    // 🧠 Suggested signals (to be implemented)
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

/// List of all signal names for dynamic connection
pub const SIGNALS: &[&str] = &[
    "build_map_start",
    "generation_progress",
    "generation_complete",
    "map_building_status",
    "tick_started",
    "tick_completed",
    "frame_budget_exceeded",
    "engine_init_start",
    "engine_initialized",
    "pipeline_start",
    "pipeline_complete",
    "sync_required",
    "rust_error",
    "map_chunk_ready",
    "tilemap_updated",
    "tilemap_sync_complete",
    "map_build_cancelled",
    "map_build_failed",
    "map_build_duration",
    "rust_extension_ready",
];
