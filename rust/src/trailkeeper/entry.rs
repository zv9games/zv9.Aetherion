/// ✅ Suggestions for trailkeeper/entry.rs

// 🔧 Add serialization support:
//     - Derive `Serialize` and `Deserialize` for `LogEntry`, `EventType`, and `LogStatus`
//     - Enables export, persistence, and interop with external tools

// 🧩 Add helper constructors:
//     - e.g. `LogEntry::new(event_type, actor, description, components, status)`
//     - Reduces boilerplate and improves ergonomics

// 🚦 Add display or formatting traits:
//     - `impl std::fmt::Display for EventType` and `LogStatus`
//     - Useful for logs, UI overlays, and diagnostics

// 📚 Document semantic meaning:
//     - Clarify how `Custom(String)` is used and what values are expected
     - Note that `actor` can be system, user, plugin, etc.

– 🧪 Add unit tests for entry creation and enum variants:
//     - Validate that `LogEntry` fields behave correctly
     - Ensure `Custom` and `ArtifactDetected` variants are parsed and displayed properly

// 🧼 Optional: Add tagging or metadata support:
//     - e.g. `tags: Vec<String>` or `context: Option<String>`
//     - Enables richer filtering and analytics

// 🚀 Future: Add severity or priority field:
//     - e.g. `priority: u8` or `severity: LogSeverity`
//     - Useful for alerting, dashboards, or triage

// 🧠 Consider exposing log schema to GDScript:
//     - Wrap in a Godot-friendly struct or export via utility node
     - Enables runtime logging and editor integration


use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum EventType {
    Build,
    Deploy,
    FileChange,
    ConfigChange,
    GitDiff,
    ArtifactDetected,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum LogStatus {
    Success,
    Failure,
    Detected,
    Ignored,
    Info,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub description: String,
    pub affected_components: Vec<String>,
    pub status: LogStatus,
}
