//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_pipeline_builder_streamer.rs

// ✅ Suggestions for aetherion/pipeline/builder/streamer.rs

// 🔧 Add dynamic pacing adjustment:
//     - Adapt `delivery_interval` based on runtime load or frame budget
//     - Could integrate with `RuntimeState::average_tick_duration()`

// 🧩 Add delivery prioritization:
//     - Support tagging chunks with priority levels
//     - e.g. urgent overlays vs background terrain

// 🚦 Add delivery feedback or retry logic:
//     - Confirm chunk was successfully received by Godot
//     - Optionally requeue failed deliveries

// 📚 Document delivery guarantees:
//     - Clarify whether chunks are guaranteed to arrive in order
//     - Note behavior under pause/resume or frame spikes

// 🧪 Add unit tests for pacing and queue behavior:
//     - Validate delivery timing, pause/resume, and queue length tracking

// 🧼 Optional: Add delivery metrics:
//     - Track total delivered chunks, average delivery rate, etc.
//     - Useful for diagnostics or performance tuning

// 🚀 Future: Add streaming modes:
//     - e.g. burst mode, lazy mode, or frame-synced mode
//     - Could expose `set_mode(StreamMode)` API

// 🧠 Consider exposing chunk preview or peek:
//     - `fn peek_next_chunk(&self) -> Option<&MapDataChunk>`
//     - Useful for debugging or editor overlays


// 🚚 Smart chunk streaming and pacing logic.
//
// Controls how and when chunks are delivered to Godot, ensuring smooth
// runtime flow and avoiding frame budget spikes.

use crate::zv9_prelude::*;

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
                self.sync.add_signal(EngineMessage::MapChunkReady);
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
// the end