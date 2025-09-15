// ✅ Suggestions for trailkeeper/macros.rs

// 🔧 Add optional fields support:
//     - Allow `affected_components` to be omitted or default to empty
//     - Could use macro overloading or default fallback

// 🧩 Add shorthand variant for simple logs:
//     - e.g. `log_event_simple!(EventType::Foo, "actor", "desc")`
//     - Improves ergonomics for frequent logging

// 🚦 Improve compile-time validation:
//     - Consider enforcing non-empty `description` or valid `status` via macro rules
//     - Prevents silent misuse or incomplete logs

// 📚 Document macro usage clearly:
//     - Include examples for typical logging scenarios
//     - Clarify that it expands to a `LogEntry` and calls `Trailkeeper::record`

// 🧪 Add macro tests or expansion checks:
//     - Validate that macro generates correct `LogEntry` structure
//     - Ensure it works across modules and with varied input types

// 🧼 Optional: Add support for structured metadata:
//     - e.g. `extra: { "key": "value" }` for future extensibility
//     - Could integrate with JSON or diagnostics overlays

// 🚀 Future: Add macro for batch logging:
//     - e.g. `log_events!(...)` for emitting multiple entries at once
//     - Useful for bulk operations or audit trails

// 🧠 Consider exposing macro to external crates:
//     - Re-export via `pub use` or document how to include it
//     - Enables broader adoption across subsystems

#[allow(unused_imports)]
use crate::zv9_prelude::*;
/// Macro for emitting structured log entries to Trailkeeper.
///
/// Supports both full and shorthand forms:
///
/// ```rust
/// log_event!(
///     event_type: EventType::System,
///     actor: "engine",
///     description: "Engine initialized",
///     affected_components: ["config", "logging"],
///     status: LogStatus::Success
/// );
///
/// log_event!(EventType::System, "engine", "Engine initialized");
/// ```
#[macro_export]
macro_rules! log_event {
    (
        event_type: $event_type:expr,
        actor: $actor:expr,
        description: $description:expr,
        affected_components: [$($component:expr),*],
        status: $status:expr
    ) => {{
        let entry = $crate::zv9__trailkeeper__entry::LogEntry {
            event_type: $event_type,
            timestamp: chrono::Utc::now(),
            actor: $actor.to_string(),
            description: $description.to_string(),
            affected_components: vec![$($component.to_string()),*],
            status: $status,
        };
        $crate::zv9_trailkeeper_collector::Trailkeeper::record(entry);
    }};

    (
        $event_type:expr,
        $actor:expr,
        $description:expr
    ) => {{
        let entry = $crate::zv9_trailkeeper_entry::LogEntry {
            event_type: $event_type,
            timestamp: chrono::Utc::now(),
            actor: $actor.to_string(),
            description: $description.to_string(),
            affected_components: vec![],
            status: $crate::zv9_trailkeeper_entry::LogStatus::Success,
        };
        $crate::zv9_trailkeeper_collector::Trailkeeper::record(entry);
    }};
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::zv9_prelude::*;

    #[test]
    fn dummy_macro_usage() {
        log_event!(
            EventType::System,
            "macro_test",
            "Macro initialized for clean build"
        );
    }
}
