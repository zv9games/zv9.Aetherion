//! Tick and budget management utilities for Aetherion.
//! Provides timing control for fixed-rate updates and runtime profiling.

use std::time::{Duration, Instant};

/// Manages fixed-rate ticking for runtime systems.
/// Tracks elapsed time and determines when a tick should occur.
pub struct TickTimer {
    last_tick: Instant,
    tick_rate: Duration,
}

impl TickTimer {
    /// Creates a new TickTimer with the given tick rate (ticks per second).
    pub fn new(ticks_per_second: u32) -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs_f64(1.0 / ticks_per_second as f64),
        }
    }

    /// Returns true if enough time has passed to trigger a tick.
    /// Resets the internal timer if a tick occurs.
    pub fn should_tick(&mut self) -> bool {
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
            true
        } else {
            false
        }
    }

    /// Returns the duration since the last tick.
    pub fn time_since_last_tick(&self) -> Duration {
        self.last_tick.elapsed()
    }

    /// Returns the configured tick rate as a duration.
    pub fn tick_interval(&self) -> Duration {
        self.tick_rate
    }
}
