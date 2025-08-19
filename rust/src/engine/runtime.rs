// runtime.rs

//! 🚀 Runtime Module
//! Drives the lifecycle loop of EchoEngine.
//! Invokes phases, tracks ticks, and supports modular extension.

use crate::engine::lifecycle::{Lifecycle, Phase, EngineCycle};
use crate::engine::dimension::BotFlipper;
use crate::utils::config::EngineConfig;

/// Runtime orchestrator
pub struct Runtime {
    pub cycle: EngineCycle,
}

impl Runtime {
    pub fn new(config: EngineConfig, use_3d: bool) -> Self {
        let flipper = BotFlipper::new(use_3d);
        let cycle = EngineCycle::new(config, flipper);
        Self { cycle }
    }

    /// Run one full lifecycle step
    pub fn step(&mut self) {
        self.cycle.advance(Phase::Generate);
        self.cycle.advance(Phase::Animate);
        self.cycle.advance(Phase::Register);
        self.cycle.advance(Phase::Tick);
    }

    /// Flip dimension mid-runtime
    pub fn flip_dimension(&mut self) {
        self.cycle.advance(Phase::FlipDimension);
    }

    /// Run for N ticks
    pub fn run(&mut self, ticks: u64) {
        self.cycle.advance(Phase::Init);
        for _ in 0..ticks {
            self.step();
        }
    }
}
