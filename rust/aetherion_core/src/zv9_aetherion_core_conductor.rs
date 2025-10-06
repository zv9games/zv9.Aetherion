#[allow(unused_imports)]
use crate::zv9_prelude::*;
use std::collections::VecDeque;
use crate::pipeline::data::{MapDataChunk, SerializableVector2i};
use crate::structure::tile_at;
use crate::zv9_util_logging::log_info;
use crate::pipeline::builder::{ChunkStreamer, ChunkDelivery};

/// 🎼 ProcCommand — procedural instructions queued for execution by the conductor.
pub enum ProcCommand {
    GenerateTerrain,
    OverlayStructure,
    ApplyModifier(Box<dyn Fn(&mut MapDataChunk) + Send>),
    EmitSignal(String),
    WaitTicks(u64),
}

/// 🎛 Conductor — orchestrates procedural flow by executing queued commands.
pub struct Conductor<D: ChunkDelivery> {
    queue: VecDeque<ProcCommand>,
    ticks_waiting: u64,
    streamer: ChunkStreamer<D>,
}

impl<D: ChunkDelivery> Conductor<D> {
    /// Creates a new conductor with an empty queue.
    pub fn new(streamer: ChunkStreamer<D>) -> Self {
        Self {
            queue: VecDeque::new(),
            ticks_waiting: 0,
            streamer,
        }
    }

    /// Adds a command to the queue.
    pub fn enqueue(&mut self, cmd: ProcCommand) {
        self.queue.push_back(cmd);
    }

    /// Executes one tick of the conductor loop.
    pub fn tick(&mut self, tick: u64, chunk: &mut MapDataChunk) {
        use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
        use std::time::{Duration, Instant};
        use rayon::prelude::*;

        log_info("conductor", &format!("🕒 Tick {} started", tick));

        if self.ticks_waiting > 0 {
            log_info("conductor", &format!("⏳ Waiting... {} ticks remaining", self.ticks_waiting));
            self.ticks_waiting -= 1;
            return;
        }

        if let Some(cmd) = self.queue.pop_front() {
            match cmd {
                ProcCommand::GenerateTerrain => {
                    log_info("conductor", "🌍 Generating terrain...");

                    let seed = tick;
                    let start = Instant::now();
                    let timeout = Duration::from_secs(30);
                    let width = 10_000;
                    let height = 100_000;
                    let chunk_size = 256;

                    let regions: Vec<(u64, u64)> = (0..height)
                        .step_by(chunk_size)
                        .flat_map(|y| (0..width).step_by(chunk_size).map(move |x| (x, y)))
                        .collect();

                    let processed = Arc::new(AtomicUsize::new(0));
                    let thread_chunks: Vec<MapDataChunk> = regions
                        .into_par_iter()
                        .map(|(x0, y0)| {
                            let mut local = MapDataChunk::new();

                            for y in y0..(y0 + chunk_size as u64).min(height) {
                                for x in x0..(x0 + chunk_size as u64).min(width) {
                                    if Instant::now().duration_since(start) >= timeout {
                                        break;
                                    }

                                    let tile = tile_at(x, y, seed);
                                    let pos = SerializableVector2i { x: x as i32, y: y as i32 };
                                    local.insert(pos, tile);
                                }
                            }

                            let count = processed.fetch_add(1, Ordering::Relaxed);
                            if count % 100 == 0 {
                                log_info("conductor", &format!("🧱 Processed {} chunks...", count));
                            }

                            local
                        })
                        .collect();

                    let total_tiles: usize = thread_chunks.iter().map(|c| c.len()).sum();
                    for thread_chunk in thread_chunks {
                        chunk.merge(thread_chunk);
                    }

                    log_info("conductor", &format!("🧨 Final tile count: {}", total_tiles));
                    self.push_status("Terrain generation complete");
                }

                ProcCommand::OverlayStructure => {
                    log_info("conductor", "🏗 Overlaying structure...");
                    // TODO: Implement structure overlay logic post-Pacman 2.0
                    self.push_status("Structure overlay complete");
                }

                ProcCommand::ApplyModifier(f) => {
                    log_info("conductor", "🖌 Applying modifier...");
                    f(chunk);
                    log_info("conductor", "🖌 Modifier applied.");
                    self.push_status("Modifier applied");
                }

                ProcCommand::EmitSignal(msg) => {
                    log_info("conductor", &format!("📢 Emitting signal: {}", msg));
                    self.push_status(&msg);
                }

                ProcCommand::WaitTicks(n) => {
                    log_info("conductor", &format!("⏳ Pausing for {} ticks...", n));
                    self.ticks_waiting = n;
                    self.push_status(&format!("Waiting {} ticks", n));
                }
            }
        }

        log_info("conductor", &format!("✅ Tick {} complete", tick));
    }

    /// Pushes a status message using the delivery trait.
    fn push_status(&mut self, msg: &str) {
        self.streamer.delivery_mut().push_status(msg);
    }

    /// Returns true if there are pending commands or active wait.
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.ticks_waiting > 0
    }

    /// Returns the number of queued commands.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Provides mutable access to the underlying streamer.
    pub fn streamer_mut(&mut self) -> &mut ChunkStreamer<D> {
        &mut self.streamer
    }
}
