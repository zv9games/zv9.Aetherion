#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;
use aetherion_shared::zv9_shared_pipeline_data_chunk::MapDataChunk;
use aetherion_shared::zv9_shared_messages::EngineMessage;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

//
// ─── Chunk Delivery Trait ──────────────────────────────────────────────────────
//

/// 📦 ChunkDelivery — trait for delivering chunks to an external system.
pub trait ChunkDelivery: Send + Clone {
    fn deliver(&mut self, chunk: MapDataChunk);
    fn sync(&mut self) -> &mut SyncBridge;

    /// 💬 Pushes a status message into the signal stream.
    fn push_status(&mut self, msg: &str) {
        let signal = EngineMessage::Status(msg.to_string());
        log::info!("📤 ChunkDelivery pushing status: {:?}", signal);
        self.sync().add_signal(signal);
    }

    /// 🧵 Returns the sync ID for tracing.
    fn sync_id(&self) -> usize;
}

//
// ─── Conductor ─────────────────────────────────────────────────────────────────
//

/// 🎛 Conductor — orchestrates procedural flow and coordinates delivery pacing.
pub struct Conductor<D: ChunkDelivery + Clone> {
    queue: VecDeque<MapDataChunk>,
    ticks_waiting: u64,
    streamer: ChunkStreamer<D>,
}

impl<D: ChunkDelivery + Clone> Conductor<D> {
    pub fn new(streamer: ChunkStreamer<D>) -> Self {
        Self {
            queue: VecDeque::new(),
            ticks_waiting: 0,
            streamer,
        }
    }

    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) {
        log::info!("📦 Conductor enqueued chunk with {} tiles", chunk.len());
        self.queue.push_back(chunk);
    }

    pub fn tick(&mut self) {
        log::info!("⏱️ Conductor tick");

        if self.ticks_waiting > 0 {
            self.ticks_waiting -= 1;
            log::info!("⏸️ Tick paused, {} remaining", self.ticks_waiting);
            return;
        }

        if let Some(chunk) = self.queue.pop_front() {
            log::info!("📤 Conductor delivering chunk with {} tiles", chunk.len());
            self.streamer.sync().add_signal(EngineMessage::ChunkReady(chunk.clone()));
            self.streamer.enqueue_chunk(chunk);
        }

        self.streamer.try_deliver();
    }

    pub fn pause(&mut self) {
        log::info!("⏸️ Conductor paused");
        self.streamer.pause();
    }

    pub fn resume(&mut self) {
        log::info!("▶️ Conductor resumed");
        self.streamer.resume();
    }

    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.streamer.has_pending()
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn streamer_mut(&mut self) -> &mut ChunkStreamer<D> {
        &mut self.streamer
    }
}

//
// ─── Chunk Streamer ────────────────────────────────────────────────────────────
//

/// 🚚 ChunkStreamer — manages pacing and delivery of chunks.
#[derive(Debug, Clone)]
pub struct ChunkStreamer<D: ChunkDelivery + Clone> {
    queue: VecDeque<MapDataChunk>,
    delivery: D,
    delivery_interval: Duration,
    last_delivery: Instant,
    paused: bool,
}

impl<D: ChunkDelivery + Clone> ChunkStreamer<D> {
    pub fn new(delivery: D, interval_ms: u64) -> Self {
        Self {
            queue: VecDeque::new(),
            delivery,
            delivery_interval: Duration::from_millis(interval_ms),
            last_delivery: Instant::now(),
            paused: false,
        }
    }

    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) {
        log::info!("📦 Streamer enqueued chunk with {} tiles", chunk.len());
        self.queue.push_back(chunk);
    }

    pub fn try_deliver(&mut self) {
        if self.paused {
            log::info!("⏸️ Streamer delivery paused");
            return;
        }

        if self.queue.is_empty() {
            log::info!("📭 Streamer queue empty");
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_delivery) >= self.delivery_interval {
            if let Some(chunk) = self.queue.pop_front() {
                log::info!("🚚 Streamer delivering chunk with {} tiles", chunk.len());
                self.delivery.deliver(chunk);
                self.last_delivery = now;
            }
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
        log::info!("⏸️ Streamer paused");
    }

    pub fn resume(&mut self) {
        self.paused = false;
        log::info!("▶️ Streamer resumed");
    }

    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn sync(&mut self) -> &mut SyncBridge {
        self.delivery.sync()
    }

    pub fn delivery_mut(&mut self) -> &mut D {
        &mut self.delivery
    }
}

//
// ─── Sync Bridge ───────────────────────────────────────────────────────────────
//

/// 🔗 SyncBridge — allows delivery backends to emit signals and coordinate with the engine.
#[derive(Default, Clone, Debug)]
pub struct SyncBridge {
    signals: Vec<EngineMessage>,
}

impl SyncBridge {
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
        }
    }

    pub fn add_signal(&mut self, signal: EngineMessage) {
        log::info!("📥 SyncBridge received signal: {:?}", signal);
        self.signals.push(signal);
    }

    pub fn drain_signals(&mut self) -> Vec<EngineMessage> {
        let drained = std::mem::take(&mut self.signals);
        log::info!("🧹 SyncBridge drained {} signals", drained.len());
        drained
    }

    pub fn has_signals(&self) -> bool {
        !self.signals.is_empty()
    }
}
