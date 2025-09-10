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
