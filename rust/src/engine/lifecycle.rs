//! 🔄 Lifecycle Module
//! Defines the operational phases of EchoEngine.
//!
//! Orchestrates generation, animation, registry updates, dimensional flips, and idle pulses.
//! Designed for modular extension, frame-safe invocation, and legacy-safe replay.
//!
//! Contributors should invoke `advance()` per phase, ideally from a paced loop or external thread.
//! Signal emission is synchronous—ensure emitters are thread-safe and deferred when bound to Godot.

use crate::engine::dimension::{BotFlipper, Position};
use crate::engine::generator::generate_field;
use crate::engine::animator::Animator;
use crate::engine::registry::Registry;
use crate::utils::config::EngineConfig;
use crate::utils::time::Tick;
use crate::interface::signal::{EchoSignal, SignalEmitter, SignalKind};
use crate::engine::dimension::DimensionContext;

/// Enum of lifecycle phases
#[derive(Clone, Debug)]
pub enum Phase {
    Init,
    Generate,
    Animate,
    Register,
    FlipDimension,
    Tick,
    Idle, // 🧘 Frame-safe observatory phase
}

/// Trait for lifecycle orchestration
pub trait Lifecycle {
    fn advance(&mut self, phase: Phase);
    fn current_tick(&self) -> Tick;
}

/// Default engine cycle
pub struct EngineCycle {
    pub tick: Tick,
    pub config: EngineConfig,
    pub flipper: BotFlipper,
    pub animator: Animator<Position>,
    pub registry: Registry,
    pub emitter: Option<Box<dyn SignalEmitter>>, // 🛎️ Optional signal hook
}

impl EngineCycle {
    /// Create a new engine cycle
    pub fn new(config: EngineConfig, flipper: BotFlipper) -> Self {
        Self {
            tick: 0,
            config,
            flipper,
            animator: Animator::new(),
            registry: Registry::new(),
            emitter: None,
        }
    }

    /// Optional: attach a signal emitter
    pub fn with_emitter(mut self, emitter: Box<dyn SignalEmitter>) -> Self {
        self.emitter = Some(emitter);
        self
    }

    /// Emit a signal if emitter is present
    fn emit(&self, signal: EchoSignal) {
        if let Some(emitter) = &self.emitter {
            emitter.emit(signal); // ⚠️ Ensure emitter is thread-safe and deferred
        }
    }
}

impl Lifecycle for EngineCycle {
    fn advance(&mut self, phase: Phase) {
        self.emit(EchoSignal::PhaseStarted(phase.clone()));

        match phase {
            Phase::Init => {
                self.tick = 0;
            }
            Phase::Generate => {
                let field = generate_field::<BotFlipper>(&self.config);
                for tile in field.tiles {
                    self.registry.upsert(tile.clone());
                    self.emit(EchoSignal::TileUpdated(tile)); // ⚠️ May flood if field is large
                }
            }
            Phase::Animate => {
                self.animator.tick(self.tick);
            }
            Phase::Register => {
                // Placeholder for future registry signals
            }
            Phase::FlipDimension => {
                self.flipper.flip();
                self.emit(EchoSignal::DimensionFlipped(self.flipper.is_3d()));
            }
            Phase::Tick => {
                self.tick += 1;
                self.emit(EchoSignal::TickAdvanced(self.tick));
            }
            Phase::Idle => {
                self.tick += 1;
                self.emit(EchoSignal::TickAdvanced(self.tick));
                // Optional: emit heartbeat glyph
            }
        }

        self.emit(EchoSignal::PhaseCompleted(phase));
    }

    fn current_tick(&self) -> Tick {
        self.tick
    }
}
