//C:/ZV9/zv9.aetherion/rust/src/godot4/interface/diagnostics.rs

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