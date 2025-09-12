//C:/ZV9/zv9.aetherion/rust/src/godot4/signals/definitions.rs

/// ✅ Suggestions for godot4/signals/definitions.rs

// 🔧 Add missing signals for full parity:
//     - Include `"chunk_ready"`, `"rust_warning"`, `"diagnostics"`, `"custom_event"`,
//       `"engine_paused"`, `"engine_resumed"`, `"engine_retry"`
//     - Ensures consistency with `AetherionSignals` and `EngineMessage`

// 🧩 Add signal metadata or categories:
//     - e.g. `pub const SIGNAL_CATEGORIES: &[(&str, &[&str])]`
//     - Enables grouped display in editor UIs or filtered connection logic

// 🚦 Add helper functions for lookup or validation:
//     - `fn is_valid_signal(name: &str) -> bool`
//     - Useful for dynamic connection or plugin systems

// 📚 Document usage patterns:
//     - Clarify how this list supports reflection, diagnostics, and tooling
//     - Note that it’s not used for actual signal emission

// 🧪 Add unit tests for signal consistency:
//     - Validate that all signals in `AetherionSignals` are listed here
//     - Prevent accidental omissions or typos

// 🧼 Optional: Add macro or derive support:
//     - e.g. `#[derive(SignalList)]` to auto-generate this list from the signal node
//     - Reduces duplication and improves maintainability

// 🚀 Future: Add localization or display names:
//     - e.g. `pub const SIGNAL_LABELS: &[(&str, &str)]`
//     - Enables user-friendly display in editor panels or logs

// 🧠 Consider exposing this list to GDScript:
//     - e.g. via a `SignalRegistry` node or exported constant
//     - Useful for dynamic UI binding or plugin development


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