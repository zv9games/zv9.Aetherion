//C:/ZV9/zv9.aetherion/rust/src/util/profiling.rs
#[allow(unused_imports)]
use crate::zv9_prelude::*;

/// ✅ Suggestions for util/profiling.rs

// 🔧 Add lightweight profiling timer:
pub struct Profiler {
    label: &'static str,
    start: std::time::Instant,
}

impl Profiler {
    pub fn start(label: &'static str) -> Self {
        Self {
            label,
            start: std::time::Instant::now(),
        }
    }

    pub fn end(self) {
        let duration = self.start.elapsed();
        println!("[⏱ {}] took {:.2?}", self.label, duration);
        // Optionally: record to Trailkeeper or diagnostics overlay
    }
}

// 🧩 Add scoped profiling macro:
#[macro_export]
macro_rules! profile_scope {
    ($label:expr) => {
        let _profiler = crate::util::profiling::Profiler::start($label);
    };
}

// 🚦 Add frame budget tracking:
pub struct FrameBudget {
    pub max_duration: std::time::Duration,
}

impl FrameBudget {
    pub fn is_exceeded(&self, elapsed: std::time::Duration) -> bool {
        elapsed > self.max_duration
    }
}

// 📚 Document usage:
//     - Clarify that this module is for runtime profiling, not statistical sampling
//     - Note that it's lightweight and suitable for in-engine diagnostics

// 🧪 Add tests for timing accuracy:
//     - Validate that `Profiler::end()` reports correct durations
//     - Ensure `FrameBudget` behaves as expected

// 🚀 Future: Add integration with Trailkeeper:
//     - Automatically log slow frames or expensive operations
//     - Enables historical performance tracking

// 🧠 Consider exposing profiling to GDScript:
//     - e.g. `ProfilerNode` that logs or displays durations
//     - Useful for editor tooling or runtime inspection




//end profiling.rs