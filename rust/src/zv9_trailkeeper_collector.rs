/// ✅ Suggestions for trailkeeper/collector.rs

// 🔧 Add thread-safe mutation utilities:
//     - `fn clear()` to reset the registry
//     - `fn count() -> usize` for diagnostics or UI display

// 🧩 Add timestamp-based filtering:
//     - e.g. `query_since(time: DateTime<Utc>)`
//     - Enables time-windowed analytics or recent event tracking

// 🚦 Improve error resilience:
//     - Replace `unwrap()` with proper error handling or logging
//     - Prevents panics in high-concurrency environments

// 📚 Document lifecycle and usage:
//     - Clarify that `LOG_REGISTRY` is global and persistent
//     - Note that `Trailkeeper` is a static utility, not an instantiable type

// 🧪 Add unit tests for record/query/all:
//     - Validate thread safety, filtering logic, and registry integrity

// 🧼 Optional: Add registry snapshot or export:
//     - `fn snapshot_json() -> String` for diagnostics or external tooling
//     - Could integrate with `export_json()` module

// 🚀 Future: Add log rotation or size limits:
//     - e.g. cap registry to N entries or rotate by time
//     - Prevents memory bloat in long-running sessions

// 🧠 Consider exposing registry to GDScript:
//     - e.g. `Trailkeeper::get_recent_logs()` via a Godot node
//     - Enables runtime inspection or editor integration


use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::zv9_prelude::*;



lazy_static! {
    static ref LOG_REGISTRY: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}


pub struct Trailkeeper;

impl Trailkeeper {
    pub fn record(entry: LogEntry) {
        let mut registry = LOG_REGISTRY.lock().unwrap();
        registry.push(entry);
    }

    pub fn query<F>(filter: F) -> Vec<LogEntry>
    where
        F: Fn(&LogEntry) -> bool,
    {
        let registry = LOG_REGISTRY.lock().unwrap();
        registry.iter().filter(|entry| filter(entry)).cloned().collect()
    }

    pub fn all() -> Vec<LogEntry> {
        let registry = LOG_REGISTRY.lock().unwrap();
        registry.clone()
    }
}
