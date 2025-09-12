//C:/ZV9/zv9.aetherion/rust/src/util/time.rs

/// ✅ Suggestions for util/time.rs

// 🔧 Add manual tick advancement:
//     - `fn force_tick(&mut self)` to manually reset `last_tick`
//     - Useful for debugging or external pacing control

// 🧩 Add tick count tracking:
//     - `tick_count: u64` to monitor how many ticks have occurred
     - Enables profiling, diagnostics, or pacing analytics

// 🚦 Improve precision control:
//     - Consider exposing `ticks_per_second()` for introspection
     - Useful for dynamic adjustment or UI display

// 📚 Document timing assumptions:
//     - Clarify that this uses monotonic time (`Instant`) and is frame-rate independent
     - Note that `should_tick()` is intended for fixed-step logic

// 🧪 Add unit tests with simulated time:
//     - Validate tick triggering, interval accuracy, and reset behavior
     - Use mock time or controlled delays for precision testing

// 🧼 Optional: Add budget tracking:
//     - `fn is_over_budget(&self, frame_duration: Duration) -> bool`
//     - Enables performance monitoring and adaptive pacing

// 🚀 Future: Add adaptive tick rate support:
//     - e.g. `fn set_tick_rate(new_rate: u32)`
//     - Enables dynamic scaling based on load or user settings

// 🧠 Consider exposing tick diagnostics:
//     - `fn describe() -> String` with interval, elapsed, and tick status
     - Useful for logging, overlays, or debugging


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

//end time.rs