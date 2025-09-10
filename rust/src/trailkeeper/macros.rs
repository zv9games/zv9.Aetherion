

#[macro_export]
macro_rules! log_event {
    (
        event_type: $event_type:expr,
        actor: $actor:expr,
        description: $description:expr,
        affected_components: [$($component:expr),*],
        status: $status:expr
    ) => {{
        use crate::trailkeeper::entry::LogEntry;
        use crate::trailkeeper::collector::Trailkeeper;
        use chrono::Utc;

        let entry = LogEntry {
            event_type: $event_type,
            timestamp: Utc::now(),
            actor: $actor.to_string(),
            description: $description.to_string(),
            affected_components: vec![$($component.to_string()),*],
            status: $status,
        };
        Trailkeeper::record(entry);
    }};
}
