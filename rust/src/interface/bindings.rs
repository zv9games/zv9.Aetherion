// bindings.rs

//! 🪢 Bindings Module
//! Exposes EchoEngine components to Godot via gdnative or godot-rust.
//! Acts as the interface ligature between engine and host.

use godot::prelude::*;
use crate::engine::dimension::Position;
use crate::interface::signal::{EchoSignal, SignalEmitter};
use std::convert::TryInto;

#[derive(GodotClass)]
#[class(base=Object, init)]
pub struct EchoBindings {
    #[export]
    tick: i64, // Godot-compatible

    #[export]
    is_3d: bool,
}

#[godot_api]
impl EchoBindings {
    #[func]
    fn flip_dimension(&mut self) {
        self.is_3d = !self.is_3d;
        let emitter = crate::interface::signal::LoggerEmitter;
        emitter.emit(EchoSignal::DimensionFlipped(self.is_3d));
    }

    #[func]
    fn advance_tick(&mut self) {
        self.tick += 1;
        let emitter = crate::interface::signal::LoggerEmitter;
        emitter.emit(EchoSignal::TickAdvanced(self.tick.try_into().unwrap())); // ← Fixed
    }
}
