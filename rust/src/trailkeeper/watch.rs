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
