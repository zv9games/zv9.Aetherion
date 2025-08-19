// lifecycle.rs

//! 🔄 Lifecycle Module
//! This module defines the operational phases of EchoEngine.
//! It orchestrates generation, animation, registry updates, and dimensional flips.
//! Designed for modular extension and legacy-safe replay.

use crate::engine::dimension::{DimensionContext, BotFlipper, Position};
use crate::engine::generator::generate_field;
use crate::engine::animator::Animator;
use crate::engine::registry::Registry;
use crate::utils::config::EngineConfig;
use crate::utils::time::Tick;

/// Enum of lifecycle phases
#[derive(Clone, Debug)]
pub enum Phase {
    Init,
    Generate,
    Animate,
    Register,
    FlipDimension,
    Tick,
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
    pub registry: Registry, // ← Fixed: removed generic argument
}

impl EngineCycle {
    pub fn new(config: EngineConfig, flipper: BotFlipper) -> Self {
        Self {
            tick: 0,
            config,
            flipper,
            animator: Animator::new(),
            registry: Registry::new(), // ← Fixed
        }
    }
}

impl Lifecycle for EngineCycle {
    fn advance(&mut self, phase: Phase) {
        match phase {
            Phase::Init => {
                self.tick = 0;
            }
            Phase::Generate => {
                let field = generate_field::<BotFlipper>(&self.config);
                for tile in field.tiles {
                    self.registry.upsert(tile);
                }
            }
            Phase::Animate => {
                self.animator.tick(self.tick);
            }
            Phase::Register => {
                // Already handled in Generate; placeholder for future signals
            }
            Phase::FlipDimension => {
                self.flipper.flip();
            }
            Phase::Tick => {
                self.tick += 1;
            }
        }
    }

    fn current_tick(&self) -> Tick {
        self.tick
    }
}
