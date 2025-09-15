/// ✅ Suggestions for trailkeeper/file_watch.rs

// 🔧 Add stop or reset functionality:
//     - `fn stop_file_watch()` to cleanly shut down the watcher
//     - Prevents dangling threads or resource leaks

// 🧩 Add support for multiple paths:
//     - Accept `Vec<String>` or `&[&str]` to watch multiple files or directories
//     - Useful for monitoring config folders or asset pipelines

// 🚦 Improve error resilience:
//     - Log retry attempts or escalate critical failures
//     - Consider exponential backoff or alerting for persistent errors

// 📚 Document debounce behavior:
//     - Clarify that changes are grouped over 2 seconds
//     - Note trade-offs between responsiveness and noise reduction

// 🧪 Add integration tests or mocks:
//     - Validate that file changes trigger correct `LogEntry` records
//     - Ensure watcher remains alive and handles edge cases

// 🧼 Optional: Add verbosity or filtering:
//     - e.g. only log `.json` or `.cfg` changes
//     - Could expose a filter closure or config struct

// 🚀 Future: Add event type classification:
//     - e.g. `Created`, `Modified`, `Deleted` distinctions in `EventType`
//     - Enables richer logging and reactive behavior

// 🧠 Consider exposing watcher state:
//     - `fn is_watching() -> bool` or `fn watched_paths() -> Vec<String>`
//     - Useful for diagnostics, UI display, or runtime control


use crate::zv9_prelude::*;
use ::notify::{RecommendedWatcher, Error, RecursiveMode}; // Disambiguated crate root
use chrono::Utc;
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, Debouncer};

use std::sync::Mutex;
use std::time::Duration;
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref FILE_WATCHER: Mutex<Option<Debouncer<RecommendedWatcher>>> = Mutex::new(None);
}

/// Starts watching one or more file paths with debounce.
pub fn start_file_watch(paths: &[&str]) {
    let watched_paths: Vec<PathBuf> = paths.iter().map(|p| PathBuf::from(p)).collect();
    let debounce_duration = Duration::from_secs(2);

    let mut debouncer = new_debouncer(debounce_duration, move |result: Result<Vec<DebouncedEvent>, Error>| {
        match result {
            Ok(events) => {
                for event in events {
                    let path_str = event.path.to_string_lossy().to_string();

                    // Optional: filter by extension
                    if !path_str.ends_with(".cfg") && !path_str.ends_with(".json") {
                        continue;
                    }

                    Trailkeeper::record(LogEntry {
                        event_type: EventType::FileChange,
                        timestamp: Utc::now(),
                        actor: "system".to_string(),
                        description: format!("File {} changed ({:?})", path_str, event.kind),
                        affected_components: vec![path_str],
                        status: LogStatus::Detected,
                    });
                }
            }
            Err(err) => {
                Trailkeeper::record(LogEntry {
                    event_type: EventType::FileChange,
                    timestamp: Utc::now(),
                    actor: "system".to_string(),
                    description: format!("Watcher error: {:?}", err),
                    affected_components: vec!["<unknown>".into()],
                    status: LogStatus::Failure,
                });
            }
        }
    }).expect("Failed to create debouncer");

    // Register paths
    for path in &watched_paths {
        debouncer.watcher().watch(path, RecursiveMode::NonRecursive)

            .expect("Failed to watch path");
    }

    let mut watcher = FILE_WATCHER.lock().unwrap();
    *watcher = Some(debouncer);
}

/// Stops the active file watcher.
pub fn stop_file_watch() {
    let mut watcher = FILE_WATCHER.lock().unwrap();
    *watcher = None;
}

/// Returns whether a watcher is currently active.
pub fn is_watching() -> bool {
    FILE_WATCHER.lock().unwrap().is_some()
}


