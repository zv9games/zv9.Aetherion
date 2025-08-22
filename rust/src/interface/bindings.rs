//! 🪢 Bindings Module
//! Exposes EchoEngine components to Godot via gdnative or godot-rust.
//!
//! Acts as the interface ligature between engine and host.
//! Designed for ABI-safe invocation, signal emission, and contributor introspection.
//!
//! Contributors may use `EchoBindings` to bridge Godot state with engine rituals.
//! All methods are frame-safe and emit signals via `LoggerEmitter`.

use godot::prelude::*;
use crate::engine::dimension::Position;
use crate::interface::signal::{EchoSignal, SignalEmitter, LoggerEmitter};
use std::convert::TryInto;

/// Godot-facing bindings for EchoEngine state
#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct EchoBindings {
    #[export]
    tick: i64, // 🕰️ Godot-compatible tick counter

    #[export]
    is_3d: bool, // 📐 Dimension flag
}

#[godot_api]
impl EchoBindings {
    /// 🔄 Flip active dimension (2D ↔ 3D)
    /// Emits `DimensionFlipped` signal with verbose logging
    #[func]
    fn flip_dimension(&mut self) {
        self.is_3d = !self.is_3d;

        let emitter = LoggerEmitter { verbose: true };
        emitter.emit(EchoSignal::DimensionFlipped(self.is_3d));
    }

    /// ⏱️ Advance tick counter
    /// Emits `TickAdvanced` signal with verbose logging
    #[func]
    fn advance_tick(&mut self) {
        self.tick += 1;

        let emitter = LoggerEmitter { verbose: true };
        emitter.emit(EchoSignal::TickAdvanced(self.tick.try_into().unwrap()));
    }
}
