/// ✅ Suggestions for trailkeeper/export.rs

// 🔧 Add error handling via `Result`:
//     - Return `Result<(), ExportError>` instead of panicking with `expect()`
//     - Enables graceful failure and integration with higher-level systems

// 🧩 Add export options:
//     - Support filtering by `EventType`, `LogStatus`, or time range
     - e.g. `export_json_filtered(path, filter: impl Fn(&LogEntry) -> bool)`

// 🚦 Add file overwrite protection:
//     - Check if file exists and confirm overwrite or append
     - Prevents accidental data loss

// 📚 Document export format:
//     - Clarify that this is a flattened JSON structure for external tools
     - Note that `title` is derived from `event_type` via `Debug`

// 🧪 Add tests with mock logs:
//     - Validate serialization output and file creation
     - Ensure timestamps and enums are formatted correctly

// 🧼 Optional: Add export timestamp or metadata:
//     - Include export time, log count, or version info in the file
     - Useful for audits and external analysis

// 🚀 Future: Add support for other formats:
//     - e.g. `export_csv(path)`, `export_yaml(path)`
//     - Enables broader compatibility with external tools

// 🧠 Consider exposing export via CLI or Godot UI:
//     - e.g. `Trailkeeper::export_to(path)` or `ExportManager` node
     - Useful for runtime diagnostics or editor integration


use crate::trailkeeper::collector::Trailkeeper;
use crate::trailkeeper::entry::LogEntry;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct SerializableLogEntry {
    title: String,
    timestamp: String,
    actor: String,
    description: String,
    affected_components: Vec<String>,
    status: String,
}

pub fn export_json(path: &str) {
    let logs: Vec<LogEntry> = Trailkeeper::all();


    let serialized: Vec<SerializableLogEntry> = logs.iter().map(|entry| SerializableLogEntry {
        title: format!("{:?}", entry.event_type),
        timestamp: entry.timestamp.to_rfc3339(),
        actor: entry.actor.clone(),
        description: entry.description.clone(),
        affected_components: entry.affected_components.clone(),
        status: format!("{:?}", entry.status),
    }).collect();

    let json = serde_json::to_string_pretty(&serialized)
        .expect("Failed to serialize logs");

    let mut file = File::create(path)
        .expect("Failed to create export file");

    file.write_all(json.as_bytes())
        .expect("Failed to write logs");
}
