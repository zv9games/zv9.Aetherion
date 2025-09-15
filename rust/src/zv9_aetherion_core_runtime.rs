//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_core_runtime.rs

// ✅ Suggestions for aetherion/core/runtime.rs

// 🔧 Add pause/resume support:
//     - `paused: bool` field
//     - `pause()`, `resume()`, `is_paused()` methods
//     - Skip tick advancement if paused

// 📈 Track last tick duration explicitly:
//     - Add `last_tick_duration: Duration`
//     - Useful for diagnostics and profiling

// 🧪 Add diagnostics snapshot struct:
//     - `RuntimeDiagnostics { tick_count, avg_tick_duration, budget, exceeded }`
//     - Return from `fn diagnostics()`

// 🧩 Support multiple tick listeners:
//     - Replace `on_tick` with `Vec<Box<dyn Fn(u64, Duration)>>`
//     - Useful for modular subsystems or plugins

// 🚦 Add tick throttling or adaptive pacing:
//     - Dynamically adjust `frame_budget` based on load or performance

// 🧼 Add logging or tracing hooks:
//     - Optional: emit tick metrics to logger or telemetry system

// 📚 Document tick smoothing formula:
//     - `(avg * 9 + elapsed) / 10` is a simple exponential moving average
//     - Consider making smoothing factor configurable

// 🚀 Future: Integrate with lifecycle.rs
//     - Only tick if lifecycle is `Running`
//     - Emit lifecycle-aware diagnostics



// 🕰 Tracks runtime state, tick progression, and frame budget.
//
// Used for scheduling, diagnostics, and signal emission across the engine.
#[allow(unused_imports)]
use crate::zv9_prelude::*;

use std::time::{Duration, Instant};
use std::fmt;

/// Tracks tick progression and frame timing for the engine runtime.
pub struct RuntimeState {
    /// Total number of ticks since startup.
    tick_count: u64,

    /// Timestamp of the last tick.
    last_tick: Instant,

    /// Maximum allowed duration per frame (e.g. 16ms for 60 FPS).
    frame_budget: Duration,

    /// Whether the last tick exceeded the frame budget.
    exceeded_budget: bool,

    /// Rolling average tick duration (for diagnostics).
    avg_tick_duration: Duration,

    /// Optional callback for tick events (e.g. signal dispatch).
    on_tick: Option<Box<dyn Fn(u64, Duration) + Send + Sync>>,
}

impl fmt::Debug for RuntimeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeState")
            .field("tick_count", &self.tick_count)
            .field("last_tick", &self.last_tick)
            .field("frame_budget", &self.frame_budget)
            .field("exceeded_budget", &self.exceeded_budget)
            .field("avg_tick_duration", &self.avg_tick_duration)
            .field("has_tick_listener", &self.on_tick.is_some())
            .finish()
    }
}

impl RuntimeState {
    /// Creates a new runtime tracker with a default frame budget (16ms ≈ 60 FPS).
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            last_tick: Instant::now(),
            frame_budget: Duration::from_millis(16),
            exceeded_budget: false,
            avg_tick_duration: Duration::ZERO,
            on_tick: None,
        }
    }

    /// Advances the tick and checks if the frame budget was exceeded.
    /// Updates internal state and invokes optional tick callback.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.exceeded_budget = elapsed > self.frame_budget;
        self.last_tick = now;
        self.tick_count += 1;

        // Update rolling average (simple smoothing)
        self.avg_tick_duration = if self.tick_count == 1 {
            elapsed
        } else {
            (self.avg_tick_duration * 9 + elapsed) / 10
        };

        // Invoke tick listener if present
        if let Some(callback) = &self.on_tick {
            callback(self.tick_count, elapsed);
        }
    }

    /// Returns true if the last frame exceeded the budget.
    pub fn is_budget_exceeded(&self) -> bool {
        self.exceeded_budget
    }

    /// Sets a custom frame budget (e.g., for low-power mode).
    pub fn set_frame_budget(&mut self, millis: u64) {
        self.frame_budget = Duration::from_millis(millis);
    }

    /// Returns the duration since the last tick.
    pub fn time_since_last_tick(&self) -> Duration {
        self.last_tick.elapsed()
    }

    /// Returns the current tick count.
    pub fn ticks(&self) -> u64 {
        self.tick_count
    }

    /// Returns the configured frame budget.
    pub fn budget(&self) -> Duration {
        self.frame_budget
    }

    /// Returns the average tick duration (smoothed).
    pub fn average_tick_duration(&self) -> Duration {
        self.avg_tick_duration
    }

    /// Registers a tick listener for diagnostics or signal dispatch.
    pub fn set_tick_listener<F>(&mut self, callback: F)
    where
        F: Fn(u64, Duration) + Send + Sync + 'static,
    {
        self.on_tick = Some(Box::new(callback));
    }

    /// Returns true if a tick listener is registered.
    pub fn has_tick_listener(&self) -> bool {
        self.on_tick.is_some()
    }
}

// the end