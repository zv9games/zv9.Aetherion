// bindings.rs

//! 🪢 Bindings Module
//! Exposes EchoEngine components to Godot via gdnative or godot-rust.
//! Acts as the interface ligature between engine and host.

use godot::prelude::*;
use crate::engine::dimension::Dimension;
use crate::engine::signal::{EchoSignal, SignalEmitter};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct EchoBindings {
    #[export]
    tick: u64,

    #[export]
    is_3d: bool,
}

#[godot_api]
impl EchoBindings {
    #[func]
    fn flip_dimension(&mut self) {
        self.is_3d = !self.is_3d;
        // Emit signal to host
        let emitter = crate::engine::signal::LoggerEmitter;
        emitter.emit(EchoSignal::DimensionFlipped(self.is_3d));
    }

    #[func]
    fn advance_tick(&mut self) {
        self.tick += 1;
        let emitter = crate::engine::signal::LoggerEmitter;
        emitter.emit(EchoSignal::TickAdvanced(self.tick));
    }

    #[func]
    fn get_tick(&self) -> u64 {
        self.tick
    }
}
