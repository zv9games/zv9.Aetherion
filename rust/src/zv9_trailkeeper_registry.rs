//c:/ZV9/zv9.aetherion/rust/src/zv9_trailkeeper_registry.rs
use crate::zv9_prelude::*;
use std::sync::Mutex;

// 🧠 Thread-safe registry of known components.
lazy_static::lazy_static! {
    static ref REGISTERED_COMPONENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

// 📦 Static list of core components for diagnostics and filtering.
pub const COMPONENTS: &[&str] = &["engine", "oracle", "map", "generator"];

// Registers a component name (no deduplication).
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

// Returns true if the component is registered.
pub fn is_registered(name: &str) -> bool {
    let components = REGISTERED_COMPONENTS.lock().unwrap();
    components.contains(&name.to_string())
}

// Returns all known event types (stubbed).
pub fn all_event_types() -> Vec<EventType> {
    vec![
        EventType::System,
        EventType::FileChange,
        EventType::StructurePlacement,
    ]
}


#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_register_and_lookup() {
        register_component("engine");
        register_component("oracle");
        register_component("map");

        assert!(is_registered("engine"));
        assert!(is_registered("oracle"));
        assert!(is_registered("map"));
        assert!(!is_registered("generator")); // not yet registered
    }

    #[test]
    fn stress_duplicate_registration() {
        register_component("engine");
        register_component("engine");
        register_component("engine");

        let components = REGISTERED_COMPONENTS.lock().unwrap();
        let count = components.iter().filter(|c| c == &"engine").count();
        assert!(count >= 3); // no deduplication
    }

    #[test]
    fn stress_bulk_registration() {
        for i in 0..1000 {
            register_component(&format!("comp_{}", i));
        }

        assert!(is_registered("comp_999"));
        assert!(!is_registered("comp_1001"));
    }

    #[test]
    fn stress_event_type_list() {
        let events = all_event_types();
        assert!(events.contains(&EventType::System));
        assert!(events.contains(&EventType::StructurePlacement));
    }
}


// the end