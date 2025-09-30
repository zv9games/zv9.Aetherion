use crate::zv9_prelude::*;
use crate::pipeline::data::MapDataChunk;
use crate::zv9_shared_messages::EngineMessage;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// 📦 Trait for delivering chunks to an external system.
pub trait ChunkDelivery: Send {
    fn deliver(&mut self, chunk: MapDataChunk);
    fn sync(&mut self) -> &mut SyncBridge;
}

/// 🎛 Orchestrates procedural flow and coordinates delivery pacing.
pub struct Conductor<D: ChunkDelivery> {
    queue: VecDeque<MapDataChunk>,
    ticks_waiting: u64,
    streamer: ChunkStreamer<D>,
}

impl<D: ChunkDelivery> Conductor<D> {
    pub fn new(streamer: ChunkStreamer<D>) -> Self {
        Self {
            queue: VecDeque::new(),
            ticks_waiting: 0,
            streamer,
        }
    }

    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) {
        self.queue.push_back(chunk);
    }

    pub fn tick(&mut self) {
        if self.ticks_waiting > 0 {
            self.ticks_waiting -= 1;
            return;
        }

        if let Some(chunk) = self.queue.pop_front() {
            self.streamer.enqueue_chunk(chunk);
        }

        self.streamer.try_deliver();
    }

    pub fn pause(&mut self) {
        self.streamer.pause();
    }

    pub fn resume(&mut self) {
        self.streamer.resume();
    }

    pub fn has_pending(&self) -> bool {
        !self.queue.is_empty() || self.streamer.has_pending()
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
}

/// 🚚 Streamer that manages chunk delivery pacing and queueing.
pub struct ChunkStreamer<D: ChunkDelivery> {
    queue: VecDeque<MapDataChunk>,
    delivery: D,
    delivery_interval: Duration,
    last_delivery: Instant,
    paused: bool,
}

impl<D: ChunkDelivery> ChunkStreamer<D> {
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
        self.queue.push_back(chunk);
    }

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

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
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
}

/// 🔗 SyncBridge allows delivery backends to emit signals and coordinate with the engine.
#[derive(Default)]
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
        self.signals.push(signal);
    }

    pub fn drain_signals(&mut self) -> Vec<EngineMessage> {
        std::mem::take(&mut self.signals)
    }

    pub fn has_signals(&self) -> bool {
        !self.signals.is_empty()
    }
}
