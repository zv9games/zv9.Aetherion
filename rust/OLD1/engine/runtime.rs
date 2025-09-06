//! 🚀 Runtime Module
//! Orchestrates the lifecycle of EchoEngine.
//!
//! Designed for modular, frame-safe invocation from Godot or external threads.
//! Contributors should call `init()` once, then `step()` per frame or spawn a backend thread.
//!
//! This module avoids host thread blocking and supports introspection, pacing, and legacy-safe extension.

use crate::engine::lifecycle::{Lifecycle, Phase, EngineCycle};
use crate::engine::dimension::BotFlipper;
use crate::utils::config::EngineConfig;

/// Runtime orchestrator
pub struct Runtime {
    pub cycle: EngineCycle,
    pub tick_count: u64, // 🕰 Tracks total ticks for introspection
}

impl Runtime {
    /// Create a new runtime instance
    pub fn new(config: EngineConfig, use_3d: bool) -> Self {
        let flipper = BotFlipper::new(use_3d);
        let cycle = EngineCycle::new(config, flipper);
        Self {
            cycle,
            tick_count: 0,
        }
    }

    /// Initialize the engine lifecycle
    /// Call once before ticking begins
    pub fn init(&mut self) {
        self.cycle.advance(Phase::Init);
    }

    /// Run one full lifecycle step
    /// Call this once per frame or from a backend thread
    pub fn step(&mut self) {
        self.cycle.advance(Phase::Generate);
        self.cycle.advance(Phase::Animate);
        self.cycle.advance(Phase::Register);
        self.cycle.advance(Phase::Tick);
        self.tick_count += 1;
    }

    /// Flip dimension mid-runtime
    pub fn flip_dimension(&mut self) {
        self.cycle.advance(Phase::FlipDimension);
    }

    /// Returns current tick count
    pub fn ticks(&self) -> u64 {
        self.tick_count
    }

    /// ❌ Deprecated: Blocking tick loop
    /// Use `init()` + `step()` or spawn a backend thread instead
    #[deprecated(note = "Use frame-safe `step()` or threaded invocation instead")]
    pub fn run(&mut self, ticks: u64) {
        self.init();
        for _ in 0..ticks {
            self.step();
        }
    }
}
