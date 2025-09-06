//C:/ZV9/zv9.aetherion/rust/src/aetherion/core/conductor.rs

//! 🎼 Runtime conductor for procedural orchestration.
//!
//! Executes commands, reacts to engine state, and manages flow between terrain,
//! structure overlays, modifiers, and signal dispatch.

use crate::aetherion::pipeline::data::MapDataChunk;
use crate::godot4::messaging::{GodotSync, EngineMessage};
use std::collections::VecDeque;

/// Procedural commands that can be queued and executed by the conductor.
pub enum ProcCommand {
    GenerateTerrain,
    OverlayStructure,
    ApplyModifier(Box<dyn Fn(&mut MapDataChunk) + Send>),
    EmitSignal(String),
    WaitTicks(u64),
}

/// Orchestrates procedural flow by executing queued commands.
pub struct Conductor {
    queue: VecDeque<ProcCommand>,
    ticks_waiting: u64,
    sync: GodotSync,
}

impl Conductor {
    /// Creates a new conductor with an empty queue and sync channel.
    pub fn new(sync: GodotSync) -> Self {
        Self {
            queue: VecDeque::new(),
            ticks_waiting: 0,
            sync,
        }
    }

    /// Adds a command to the queue.
    pub fn enqueue(&mut self, cmd: ProcCommand) {
        self.queue.push_back(cmd);
    }

    /// Processes one tick of the conductor loop.
    pub fn tick(&mut self, tick: u64, chunk: &mut MapDataChunk) {
        if self.ticks_waiting > 0 {
            self.ticks_waiting -= 1;
            return;
        }

        if let Some(cmd) = self.queue.pop_front() {
            match cmd {
                ProcCommand::GenerateTerrain => {
                    self.sync.add_signal(EngineMessage::Status("🌍 Generating terrain...".to_string()));
                    // TODO: Trigger terrain generation logic here
                }
                ProcCommand::OverlayStructure => {
                    self.sync.add_signal(EngineMessage::Status("🏗 Overlaying structure...".to_string()));
                    // TODO: Trigger structure overlay logic here
                }
                ProcCommand::ApplyModifier(f) => {
                    f(chunk);
                    self.sync.add_signal(EngineMessage::Status("🖌 Modifier applied.".to_string()));
                }
                ProcCommand::EmitSignal(msg) => {
                    self.sync.add_signal(EngineMessage::Status(msg));
                }
                ProcCommand::WaitTicks(n) => {
                    self.ticks_waiting = n;
                }
            }
        }
    }

    /// Returns true if the conductor has pending commands.
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.ticks_waiting > 0
    }

    /// Returns the number of queued commands.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
}

//end conductor.rs