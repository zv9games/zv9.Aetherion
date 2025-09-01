//! Tracks runtime state, tick progression, and frame budget.
//! Used for scheduling, diagnostics, and signal emission across the engine.

use std::time::{Duration, Instant};

/// Tracks tick progression and frame timing for the engine runtime.
#[derive(Debug)]
pub struct RuntimeState {
    /// Total number of ticks since startup.
    pub tick_count: u64,

    /// Timestamp of the last tick.
    pub last_tick: Instant,

    /// Maximum allowed duration per frame (e.g. 16ms for 60 FPS).
    pub frame_budget: Duration,

    /// Whether the last tick exceeded the frame budget.
    pub exceeded_budget: bool,
}

impl RuntimeState {
    /// Creates a new runtime tracker with a default frame budget (16ms ≈ 60 FPS).
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            last_tick: Instant::now(),
            frame_budget: Duration::from_millis(16),
            exceeded_budget: false,
        }
    }

    /// Advances the tick and checks if the frame budget was exceeded.
    /// Updates internal state and tick count.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.exceeded_budget = elapsed > self.frame_budget;
        self.last_tick = now;
        self.tick_count += 1;

        // Future: emit "tick_started", "tick_completed", or "frame_budget_exceeded"
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
}
