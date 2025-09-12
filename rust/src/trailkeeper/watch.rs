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


use crate::trailkeeper::entry::{LogEntry, EventType, LogStatus};
use crate::trailkeeper::collector::Trailkeeper;
use chrono::Utc;
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, Debouncer};
use std::sync::Mutex;
use std::time::Duration;

// Persistent debouncer to keep the watcher alive
lazy_static::lazy_static! {
    static ref FILE_WATCHER: Mutex<Option<Debouncer<notify::RecommendedWatcher>>> = Mutex::new(None);
}

pub fn start_file_watch(path: &str) {
    let path_owned = path.to_string();
    let path_for_closure = path_owned.clone();
    let debounce_duration = Duration::from_secs(2);

    let debouncer = new_debouncer(debounce_duration, move |result: Result<Vec<DebouncedEvent>, notify::Error>| {
        match result {
            Ok(events) => {
                for event in events {
                    Trailkeeper::record(LogEntry {
                        event_type: EventType::FileChange,
                        timestamp: Utc::now(),
                        actor: "system".to_string(),
                        description: format!(
                            "File {} changed ({:?})",
                            event.path.to_string_lossy(),
                            event.kind
                        ),
                        affected_components: vec![event.path.to_string_lossy().to_string()],
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
                    affected_components: vec![path_for_closure.clone()],
                    status: LogStatus::Failure,
                });
            }
        }
    }).expect("Failed to create debouncer");

    // Store the debouncer to keep it alive
    let mut watcher = FILE_WATCHER.lock().unwrap();
    *watcher = Some(debouncer);
}
