//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_pipeline_builder_streamer.rs



use crate::zv9_prelude::*;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// 🚚 Controls pacing and prioritization of chunk delivery.
pub struct ChunkStreamer {
    queue: VecDeque<MapDataChunk>,
    sync: GodotSync,
    delivery_interval: Duration,
    last_delivery: Instant,
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


#[cfg(test)]
mod stress_tests {
    use super::*;
    use crate::zv9_prelude::*;

    #[test]
    fn stress_enqueue_and_deliver() {
        let sync = GodotSync::init();
        let mut streamer = ChunkStreamer::new(sync, 1);
        let chunk = MapDataChunk::default();

        for _ in 0..1000 {
            streamer.enqueue_chunk(chunk.clone());
        }

        for _ in 0..1000 {
            std::thread::sleep(Duration::from_millis(1));
            streamer.try_deliver();
        }

        assert_eq!(streamer.queue_len(), 0);
        assert!(!streamer.has_pending());
    }

    #[test]
    fn stress_pause_resume_behavior() {
        let sync = GodotSync::init();
        let mut streamer = ChunkStreamer::new(sync, 1);
        let chunk = MapDataChunk::default();

        streamer.enqueue_chunk(chunk.clone());
        streamer.pause();
        std::thread::sleep(Duration::from_millis(5));
        streamer.try_deliver();

        assert_eq!(streamer.queue_len(), 1); // Should not deliver while paused

        streamer.resume();
        std::thread::sleep(Duration::from_millis(5));
        streamer.try_deliver();

        assert_eq!(streamer.queue_len(), 0); // Should deliver after resume
    }

    #[test]
    fn stress_delivery_interval_enforcement() {
        let sync = GodotSync::init();
        let mut streamer = ChunkStreamer::new(sync, 50); // 50ms interval
        let chunk = MapDataChunk::default();

        streamer.enqueue_chunk(chunk.clone());
        streamer.try_deliver(); // Should not deliver immediately

        assert_eq!(streamer.queue_len(), 1);

        std::thread::sleep(Duration::from_millis(60));
        streamer.try_deliver(); // Should deliver now

        assert_eq!(streamer.queue_len(), 0);
    }
}


// the end