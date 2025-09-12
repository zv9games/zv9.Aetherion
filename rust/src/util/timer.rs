/// ✅ Suggestions for util/timer.rs

// 🔧 Add convenience methods:
//     - `fn elapsed_secs(&self) -> f64`
//     - `fn elapsed_millis(&self) -> u128`
//     - Useful for logging, diagnostics, and performance metrics

// 🧩 Add pause/resume support:
//     - `fn pause()`, `fn resume()` with internal offset tracking
//     - Enables controlled timing in animations or simulations

// 🚦 Improve precision control:
//     - Consider exposing `Instant` directly via `fn started_at() -> Instant`
//     - Useful for profiling or syncing with external systems

// 📚 Document intended use cases:
//     - Clarify that this is a monotonic timer for measuring durations
//     - Note that it’s not suitable for wall-clock or scheduled events

// 🧪 Add unit tests for `elapsed()` and `reset()`:
//     - Validate time progression and reset behavior
//     - Ensure consistency across platforms

// 🧼 Optional: Add `is_expired(duration: Duration)` helper:
//     - e.g. `fn is_expired(&self, timeout: Duration) -> bool`
//     - Useful for polling loops, timeouts, and pacing logic

// 🚀 Future: Add named or labeled timers:
//     - e.g. `pub struct LabeledTimer { label: String, timer: Timer }`
//     - Enables structured profiling or grouped diagnostics

// 🧠 Consider exposing timer to GDScript:
//     - Wrap in a Godot-friendly node or utility class
//     - Useful for runtime performance tracking or animation pacing


use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}
