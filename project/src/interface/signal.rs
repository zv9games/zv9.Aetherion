// signal.rs

//! 🔔 Signal Module
//! Emits lifecycle events and tile updates from EchoEngine.
//! Designed for Godot, CLI, or multiplayer integration.

use crate::engine::types::{Tile, TileKind};
use crate::engine::dimension::Position;
use crate::engine::lifecycle::Phase;

/// Signal types emitted by the engine
#[derive(Clone, Debug)]
pub enum EchoSignal {
    PhaseStarted(Phase),
    PhaseCompleted(Phase),
    TileUpdated(Tile),
    DimensionFlipped(bool), // true = 3D, false = 2D
    TickAdvanced(u64),
}

/// Signal emitter trait
pub trait SignalEmitter {
    fn emit(&self, signal: EchoSignal);
}

/// Default logger-based emitter
pub struct LoggerEmitter;

impl SignalEmitter for LoggerEmitter {
    fn emit(&self, signal: EchoSignal) {
        println!("[EchoSignal] {:?}", signal);
    }
}
