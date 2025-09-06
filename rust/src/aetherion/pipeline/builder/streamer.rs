//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/builder/streamer.rs

//! 🚚 Smart chunk streaming and pacing logic.
//!
//! Controls how and when chunks are delivered to Godot, ensuring smooth
//! runtime flow and avoiding frame budget spikes.

use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::aetherion::pipeline::data::chunk::MapDataChunk;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Controls pacing and prioritization of chunk delivery.
pub struct ChunkStreamer {
    /// Queue of chunks waiting to be delivered.
    queue: VecDeque<MapDataChunk>,

    /// Sync channel to Godot.
    sync: GodotSync,

    /// Minimum delay between chunk deliveries.
    delivery_interval: Duration,

    /// Timestamp of last delivery.
    last_delivery: Instant,

    /// Whether streaming is paused.
    paused: bool,
}

impl ChunkStreamer {
    /// Creates a new streamer with a default pacing interval.
    pub fn new(sync: GodotSync, interval_ms: u64) -> Self {
        Self {
            queue: VecDeque::new(),
            sync,
            delivery_interval: Duration::from_millis(interval_ms),
            last_delivery: Instant::now(),
            paused: false,
        }
    }

    /// Adds a chunk to the delivery queue.
    pub fn enqueue_chunk(&mut self, chunk: MapDataChunk) {
        self.queue.push_back(chunk);
    }

    /// Attempts to deliver a chunk if pacing allows.
    pub fn try_deliver(&mut self) {
        if self.paused || self.queue.is_empty() {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_delivery) >= self.delivery_interval {
            if let Some(chunk) = self.queue.pop_front() {
                self.sync.add_chunk(chunk);
                self.last_delivery = now;
            }
        }
    }

    /// Pauses chunk delivery.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes chunk delivery.
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

    /// Returns a reference to the underlying GodotSync channel.
    pub fn sync(&self) -> &GodotSync {
        &self.sync
    }

    /// Returns a mutable reference to the underlying GodotSync channel.
    pub fn sync_mut(&mut self) -> &mut GodotSync {
        &mut self.sync
    }
}

//end streamer.rs