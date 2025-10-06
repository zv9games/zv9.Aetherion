
#[allow(unused_imports)]
use crate::zv9_prelude::*;
use crate::pipeline::data::MapDataChunk;
use crate::zv9_shared_messages::EngineMessage;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

//
// ─── Chunk Delivery Trait ──────────────────────────────────────────────────────
//

/// 📦 ChunkDelivery — trait for delivering chunks to an external system.
pub trait ChunkDelivery: Send {
    fn deliver(&mut self, chunk: MapDataChunk);
    fn sync(&mut self) -> &mut SyncBridge;

    /// 💬 Pushes a status message into the signal stream.
    fn push_status(&mut self, msg: &str) {
        self.sync().add_signal(EngineMessage::Status(msg.to_string()));
    }
}

//
// ─── Conductor ─────────────────────────────────────────────────────────────────
//

/// 🎛 Conductor — orchestrates procedural flow and coordinates delivery pacing.
pub struct Conductor<D: ChunkDelivery> {
    queue: VecDeque<MapDataChunk>,
    ticks_waiting: u64,
    streamer: ChunkStreamer<D>,
}

impl<D: ChunkDelivery> Conductor<D> {
    /// Creates a new conductor with a delivery streamer.
    pub fn new(streamer: ChunkStreamer<D>) -> Self {
        Self {
            queue: VecDeque::new(),
            ticks_waiting: 0,
            streamer,
        }
    }

    /// Queues a chunk for future delivery.
    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) {
        self.queue.push_back(chunk);
    }

    /// Advances the conductor by one tick.
    pub fn tick(&mut self) {
        if self.ticks_waiting > 0 {
            self.ticks_waiting -= 1;
            return;
        }

        if let Some(chunk) = self.queue.pop_front() {
			self.streamer.sync().add_signal(EngineMessage::ChunkReady(chunk.clone()));
			self.streamer.enqueue_chunk(chunk);
		}


        self.streamer.try_deliver();
    }

    /// Pauses delivery.
    pub fn pause(&mut self) {
        self.streamer.pause();
    }

    /// Resumes delivery.
    pub fn resume(&mut self) {
        self.streamer.resume();
    }

    /// Returns true if there are pending chunks or active wait.
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.streamer.has_pending()
    }

    /// Returns the number of queued chunks.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Provides mutable access to the underlying streamer.
    pub fn streamer_mut(&mut self) -> &mut ChunkStreamer<D> {
        &mut self.streamer
    }
}

//
// ─── Chunk Streamer ────────────────────────────────────────────────────────────
//

/// 🚚 ChunkStreamer — manages pacing and delivery of chunks.
pub struct ChunkStreamer<D: ChunkDelivery> {
    queue: VecDeque<MapDataChunk>,
    delivery: D,
    delivery_interval: Duration,
    last_delivery: Instant,
    paused: bool,
}

impl<D: ChunkDelivery> ChunkStreamer<D> {
    /// Creates a new streamer with a delivery backend and interval.
    pub fn new(delivery: D, interval_ms: u64) -> Self {
        Self {
            queue: VecDeque::new(),
            delivery,
            delivery_interval: Duration::from_millis(interval_ms),
            last_delivery: Instant::now(),
            paused: false,
        }
    }

    /// Queues a chunk for delivery.
    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) {
        self.queue.push_back(chunk);
    }

    /// Attempts to deliver a chunk if interval has passed.
    pub fn try_deliver(&mut self) {
        if self.paused || self.queue.is_empty() {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_delivery) >= self.delivery_interval {
            if let Some(chunk) = self.queue.pop_front() {
                self.delivery.deliver(chunk);
                self.last_delivery = now;
            }
        }
    }

    /// Pauses delivery.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes delivery.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Returns true if there are pending chunks.
    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty()
    }

    /// Returns the number of queued chunks.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Accesses the sync bridge.
    pub fn sync(&mut self) -> &mut SyncBridge {
        self.delivery.sync()
    }

    /// Provides mutable access to the delivery backend.
    pub fn delivery_mut(&mut self) -> &mut D {
        &mut self.delivery
    }
}

//
// ─── Sync Bridge ───────────────────────────────────────────────────────────────
//

/// 🔗 SyncBridge — allows delivery backends to emit signals and coordinate with the engine.
#[derive(Default)]
pub struct SyncBridge {
    signals: Vec<EngineMessage>,
}

impl SyncBridge {
    /// Creates a new sync bridge.
    pub fn new() -> Self {
        Self {
            signals: Vec::new(),
        }
    }

    /// Queues a signal message.
    pub fn add_signal(&mut self, signal: EngineMessage) {
        self.signals.push(signal);
    }

    /// Retrieves and clears all queued signals.
    pub fn drain_signals(&mut self) -> Vec<EngineMessage> {
        std::mem::take(&mut self.signals)
    }

    /// Returns true if there are pending signals.
    pub fn has_signals(&self) -> bool {
        !self.signals.is_empty()
    }
}
