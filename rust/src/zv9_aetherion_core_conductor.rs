//c:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_core_conductor.rs




use crate::zv9_prelude::*;
use std::collections::VecDeque;

/// 🎼 Procedural commands that can be queued and executed by the conductor.
pub enum ProcCommand {
    GenerateTerrain,
    OverlayStructure,
    ApplyModifier(Box<dyn Fn(&mut MapDataChunk) + Send>),
    EmitSignal(String),
    WaitTicks(u64),
}

/// 🎛 Orchestrates procedural flow by executing queued commands.
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
    use std::time::{Instant, Duration};
    use rayon::prelude::*;

    log_info("conductor", &format!("🎼 Tick {} processed", tick));
    self.sync.add_signal(EngineMessage::Status(format!("🎼 Tick {} processed", tick)));

    if self.ticks_waiting > 0 {
        log_info("conductor", &format!("⏳ Waiting... {} ticks remaining", self.ticks_waiting));
        self.ticks_waiting -= 1;
        return;
    }

    if let Some(cmd) = self.queue.pop_front() {
        match cmd {
            ProcCommand::GenerateTerrain => {
                log_info("conductor", "🌍 Generating terrain...");
                self.sync.add_signal(EngineMessage::Status("🌍 Generating terrain...".into()));

                let seed = tick;
                let start = Instant::now();
                let limit = Duration::from_secs(30);
                let width = 10_000;
                let height = 100_000;
                let chunk_size = 256;

                let chunks: Vec<(u64, u64)> = (0..height)
                    .step_by(chunk_size)
                    .flat_map(|y| (0..width).step_by(chunk_size).map(move |x| (x, y)))
                    .collect();

                let thread_chunks: Vec<MapDataChunk> = chunks
                    .into_par_iter()
                    .map(|(x0, y0)| {
                        let mut local_chunk = MapDataChunk::new();

                        for y in y0..(y0 + chunk_size as u64).min(height) {
							for x in x0..(x0 + chunk_size as u64).min(width) {
                                if Instant::now() - start >= limit {
                                    break;
                                }

                                let tile = tile_at(x, y, seed);
                                let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                                local_chunk.insert(pos, tile);
                            }
                        }

                        local_chunk
                    })
                    .collect();

                let mut total_tiles = 0;
                for thread_chunk in thread_chunks {
                    total_tiles += thread_chunk.len();
                    chunk.merge(thread_chunk);
                }

                log_info("conductor", &format!("🧨 Final tile count: {}", total_tiles));
                self.sync.add_signal(EngineMessage::Status(format!("🧨 Final tile count: {}", total_tiles)));
            }

            ProcCommand::OverlayStructure => {
                log_info("conductor", "🏗 Overlaying structure...");
                self.sync.add_signal(EngineMessage::Status("🏗 Overlaying structure...".into()));
                // TODO: Implement structure overlay
            }

            ProcCommand::ApplyModifier(f) => {
                log_info("conductor", "🖌 Applying modifier...");
                f(chunk);
                self.sync.add_signal(EngineMessage::Status("🖌 Modifier applied.".into()));
            }

            ProcCommand::EmitSignal(msg) => {
                log_info("conductor", &format!("📢 Emitting signal: {}", msg));
                self.sync.add_signal(EngineMessage::Status(msg));
            }

            ProcCommand::WaitTicks(n) => {
                log_info("conductor", &format!("⏳ Pausing for {} ticks...", n));
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

//
// ─── Stress Tests ─────────────────────────────────────────────────────────────
//

#[cfg(test)]
mod stress_tests {
    use super::*;
    use crate::zv9_prelude::*;

    #[test]
    fn stress_enqueue_and_tick() {
        let sync = GodotSync::init();
        let mut conductor = Conductor::new(sync);
        let mut chunk = MapDataChunk::default();

        for _ in 0..10_000 {
            conductor.enqueue(ProcCommand::EmitSignal("Stress test signal".into()));
        }

        for tick in 0..10_000 {
            conductor.tick(tick, &mut chunk);
        }

        assert_eq!(conductor.queue_len(), 0);
        assert!(!conductor.has_pending());
    }

    #[test]
    fn stress_wait_logic() {
        let sync = GodotSync::init();
        let mut conductor = Conductor::new(sync);
        let mut chunk = MapDataChunk::default();

        conductor.enqueue(ProcCommand::WaitTicks(100));
        conductor.enqueue(ProcCommand::EmitSignal("After wait".into()));

        for tick in 0..150 {
            conductor.tick(tick, &mut chunk);
        }

        assert!(!conductor.has_pending());
    }

    #[test]
    fn stress_modifier_application() {
        let sync = GodotSync::init();
        let mut conductor = Conductor::new(sync);
        let mut chunk = MapDataChunk::default();

        conductor.enqueue(ProcCommand::ApplyModifier(Box::new(|chunk| {
            for _ in 0..1000 {
                chunk.apply_noise(0.5); // Replace with actual logic
            }
        })));

        conductor.tick(0, &mut chunk);
        assert!(!conductor.has_pending());
    }
}


// the end