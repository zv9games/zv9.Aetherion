//C:/ZV9/zv9.aetherion/rust/src/godot4/signals/definitions.rs

// 📡 Aetherion Signal Definitions — Centralized list for dynamic access and tooling.
// This module does not define the signal node itself, but supports reflection, connection, and diagnostics.

/// List of all signal names emitted by `AetherionSignals`.
/// Useful for dynamic connection, logging, or editor tooling.
pub const SIGNALS: &[&str] = &[
    // ✅ Core generation signals
    "build_map_start",
    "generation_progress",
    "generation_complete",
    "map_building_status",

    // 🧠 Lifecycle & diagnostics
    "tick_started",
    "tick_completed",
    "frame_budget_exceeded",
    "engine_init_start",
    "engine_initialized",
    "pipeline_start",
    "pipeline_complete",
    "sync_required",
    "rust_error",

    // 🔁 Tilemap & map events
    "map_chunk_ready",
    "tilemap_updated",
    "tilemap_sync_complete",
    "map_build_cancelled",
    "map_build_failed",
    "map_build_duration",
    "rust_extension_ready",
];

//end definitions.rs