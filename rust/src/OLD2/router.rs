use crate::godot_bridge::signals::AetherionSignals;
use crate::utilities::concurrency::EngineMessage;
use godot::prelude::*;

pub struct SignalRouter;

impl SignalRouter {
    /// Routes engine messages to the appropriate Godot signals.
    pub fn route(signals: &mut Gd<AetherionSignals>, msg: &EngineMessage) {
        match msg {
            EngineMessage::Progress(p) => {
                signals.call_deferred("emit_generation_progress", &[p.to_variant()]);
            }
            EngineMessage::Complete { width, height, mode, animate, duration } => {
                let mut dict = Dictionary::new();
                dict.insert("width", *width);
                dict.insert("height", *height);
                dict.insert("mode", mode.clone());
                dict.insert("animate", *animate);
                dict.insert("duration", *duration);
                signals.call_deferred("emit_generation_complete", &[dict.to_variant()]);
            }
            _ => {
                godot_warn!("⚠️ Unhandled EngineMessage in SignalRouter.");
            }
        }
    }
}
