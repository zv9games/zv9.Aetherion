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

/// Signal kind — used for filtering, mapping, or routing
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SignalKind {
    Phase,
    Tile,
    Dimension,
    Tick,
}

impl EchoSignal {
    /// Extracts the kind of signal for introspection
    pub fn kind(&self) -> SignalKind {
        match self {
            EchoSignal::PhaseStarted(_) | EchoSignal::PhaseCompleted(_) => SignalKind::Phase,
            EchoSignal::TileUpdated(_) => SignalKind::Tile,
            EchoSignal::DimensionFlipped(_) => SignalKind::Dimension,
            EchoSignal::TickAdvanced(_) => SignalKind::Tick,
        }
    }
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
