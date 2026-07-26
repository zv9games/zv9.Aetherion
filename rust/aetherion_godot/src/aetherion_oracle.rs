// aetherion_godot/src/aetherion_oracle.rs

use godot::prelude::*;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use crate::aetherion_engine::AetherionEngine;
use tracing::{info, warn}; // Use tracing for standardized logging

/// 🔮 AetherionOracle — Godot-facing node for manually driving the AetherionEngine
/// and providing complex query/lookup logic (future state).
#[derive(GodotClass)]
#[class(tool, base = Node, init)]
pub struct AetherionOracle {
    #[base]
    base: Base<Node>,
    /// Link to the AetherionEngine instance. Stored as Gd<AetherionEngine> for safety.
    engine: Option<Gd<AetherionEngine>>,
    tick_count: u64,
}

impl AetherionOracle {
    pub fn init(base: Base<Node>) -> Self {
        Self {
            base,
            engine: None,
            tick_count: 0
        }
    }
}

#[godot_api]
impl AetherionOracle {

    /// Called when the node enters the scene tree.
    #[func]
    fn _ready(&mut self) {
        info!("AetherionOracle is online. I await the ignition.");
        self.base_mut().set_process(true);
    }

    /// Links the Oracle to a target engine node.
    #[func]
    pub fn set_engine(&mut self, engine: Gd<AetherionEngine>) {
        self.engine = Some(engine);
        info!("Oracle: Engine link established.");
    }

    /// Sends a tick to the linked engine.
    /// FIX: Switched from `engine.call()` to the type-safe `bind_mut().tick()`.
    #[func]
    pub fn tick(&mut self) {
        match self.engine.as_mut() {
            Some(engine) => {
                // Direct call to the Rust-defined method, passing the current tick count.
                engine.bind_mut().tick(self.tick_count);

                self.tick_count += 1;
            }
            None => {
                warn!("Oracle: No engine linked. Tick aborted.");
            }
        }
    }

    /// Responds to a ping from external systems.
    #[func]
    pub fn ping(&self) {
        info!("Oracle: Ping received. I am awake.");
    }

    /// Resets the internal tick counter.
    #[func]
    pub fn reset(&mut self) {
        self.tick_count = 0;
        info!("Oracle: Tick counter reset.");
    }

    /// Returns the current tick count.
    #[func]
    pub fn get_tick(&self) -> u64 {
        self.tick_count
    }
}