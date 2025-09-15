
// 🔧 Add registry for known components or subsystems:
//     - e.g. `pub const COMPONENTS: &[&str] = &["engine", "oracle", "map", "generator"]`
//     - Enables filtering, grouping, and UI display

// 🧩 Add dynamic registration API:
//     - `fn register_component(name: &str)`
//     - `fn is_registered(name: &str) -> bool`
//     - Useful for plugin systems or runtime extension

// 🚦 Add event type introspection:
//     - `fn all_event_types() -> Vec<EventType>`
//     - Enables editor tooling, analytics, or filtering

// 📚 Document registry purpose:
//     - Clarify whether it’s for validation, filtering, display, or analytics
//     - Note how it integrates with `Trailkeeper` and `LogEntry`

// 🧪 Add tests for registration and lookup:
//     - Validate component registration, duplicate handling, and lookup behavior

// 🧼 Optional: Add metadata for components:
//     - e.g. `ComponentInfo { name, description, tags }`
//     - Enables richer UI display or filtering

// 🚀 Future: Add support for log source tagging:
//     - e.g. `enum LogSource { System, User, Plugin(String) }`
//     - Enables attribution and filtering in dashboards

// 🧠 Consider exposing registry to GDScript:
//     - e.g. `fn get_registered_components() -> Array<GString>`
//     - Useful for editor integration or runtime scripting
use crate::zv9_prelude::*;
use std::sync::Mutex;


lazy_static::lazy_static! {
    static ref REGISTERED_COMPONENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}


/// Static list of known components for filtering and diagnostics.
pub const COMPONENTS: &[&str] = &["engine", "oracle", "map", "generator"];


/// Registers a component name (no deduplication).
pub fn register_component(name: &str) {
    let mut components = REGISTERED_COMPONENTS.lock().unwrap();
    components.push(name.to_string());

    Trailkeeper::record(LogEntry {
        event_type: EventType::System,
        timestamp: chrono::Utc::now(),
        actor: "registry".into(),
        description: format!("Registered component: {}", name),
        affected_components: vec![name.into()],
        status: LogStatus::Success,
    });
}

pub fn is_registered(name: &str) -> bool {
    let components = REGISTERED_COMPONENTS.lock().unwrap();
    components.contains(&name.to_string())
}

/// Returns all known event types (stubbed).
pub fn all_event_types() -> Vec<EventType> {
    vec![
        EventType::System,
        EventType::FileChange,
        EventType::StructurePlacement,
    ]
}
