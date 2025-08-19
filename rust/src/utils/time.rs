//! 🕰️ Time Utilities
//! This module defines tick-based timing types used across lifecycle orchestration.
//! It is dimension-agnostic and frame-native, designed for modular reuse.

/// A single frame tick, used as the atomic unit of time.
pub type Tick = u64;

/// A duration measured in ticks (frames).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration {
    pub frames: Tick,
}

impl Duration {
    /// Creates a new duration from a number of frames.
    pub fn new(frames: Tick) -> Self {
        Self { frames }
    }

    /// Returns the duration in seconds, assuming a fixed frame rate.
    pub fn as_seconds(&self, fps: Tick) -> f64 {
        self.frames as f64 / fps as f64
    }

    /// Adds two durations together.
    pub fn add(&self, other: Duration) -> Duration {
        Duration::new(self.frames + other.frames)
    }

    /// Returns true if this duration is zero.
    pub fn is_zero(&self) -> bool {
        self.frames == 0
    }
}
