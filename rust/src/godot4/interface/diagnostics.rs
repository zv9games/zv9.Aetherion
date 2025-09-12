//C:/ZV9/zv9.aetherion/rust/src/godot4/interface/diagnostics.rs

/// ✅ Suggestions for godot4/interface/diagnostics.rs

// 🔧 Add more runtime metrics:
//     - Memory usage, thread count, FPS, tilemap size, etc.
//     - Could expose `fn update_extended_metrics(...)` or use a `Dictionary` input

// 🧩 Add configurable display options:
//     - Font size, color, position, visibility toggle
//     - Useful for adapting to different editor layouts or runtime modes

// 🚦 Add bounds or threshold alerts:
//     - Highlight metrics in red if `avg_tick` exceeds budget or `queue_len` spikes
//     - Could emit signals or change label styling dynamically

// 📚 Document intended usage:
//     - Clarify that this node is meant for overlay diagnostics in Godot
//     - Note how it integrates with `AetherionEngine` or other systems

// 🧪 Add integration tests or GDScript examples:
//     - Validate that metrics update correctly and formatting is stable

// 🧼 Optional: Add refresh rate control:
//     - e.g. `fn set_update_interval(ms: i32)`
//     - Prevent excessive updates or allow throttling

// 🚀 Future: Add support for multi-line or tabbed overlays:
//     - e.g. separate panels for performance, memory, and generation status
//     - Could integrate with a `DiagnosticsPanel` or `UIManager`

// 🧠 Consider exposing metrics as signals:
//     - `signal metrics_updated(tick, avg_tick, queue_len)`
//     - Enables external listeners or plugin hooks


use godot::prelude::*;
use godot::classes::Label;

#[derive(GodotClass)]
#[class(init, base = Label)]
pub struct DiagnosticsOverlay {
    #[base]
    base: Base<Label>,
}

#[godot_api]
impl DiagnosticsOverlay {
    fn init(base: Base<Label>) -> Self {
        Self { base }
    }

    #[func]
    fn update_metrics(&mut self, tick: u64, avg_tick: f64, queue_len: i32) {
        let text = format!(
            "🧠 Tick: {}\n⏱ Avg Tick Duration: {:.2}ms\n📦 Chunk Queue: {}",
            tick,
            avg_tick,
            queue_len
        );
        self.base.set_text(text.into());
    }
}

//end diagnostics.rs