//! 🔔 Signal Module
//! Emits lifecycle events and tile updates from EchoEngine.
//!
//! Designed for Godot, CLI, or multiplayer integration.
//! Signals are synchronous by default—ensure emitters are thread-safe and deferred when bound to UI threads.
//!
//! Contributors may filter, route, or introspect signals using `SignalKind` and `EchoSignal::label()`.

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

    /// Optional: returns a short label for UI overlays or logs
    pub fn label(&self) -> &'static str {
        match self {
            EchoSignal::PhaseStarted(_) => "PhaseStarted",
            EchoSignal::PhaseCompleted(_) => "PhaseCompleted",
            EchoSignal::TileUpdated(_) => "TileUpdated",
            EchoSignal::DimensionFlipped(_) => "DimensionFlipped",
            EchoSignal::TickAdvanced(_) => "TickAdvanced",
        }
    }
}

/// Trait for signal emission
/// Implementations must be thread-safe and non-blocking when used in UI contexts
pub trait SignalEmitter {
    fn emit(&self, signal: EchoSignal);
}

/// Default logger-based emitter
/// Emits signals to stdout for debugging and introspection
pub struct LoggerEmitter {
    pub verbose: bool, // 🪶 Optional toggle for verbosity
}

impl Default for LoggerEmitter {
    fn default() -> Self {
        Self { verbose: true }
    }
}

impl SignalEmitter for LoggerEmitter {
    fn emit(&self, signal: EchoSignal) {
        if self.verbose {
            println!("[EchoSignal] {:?}", signal);
        }
    }
}
