// logger.rs

//! 📡 Logger Module
//! Records engine events, signals, and audit traces.
//! Pluggable backend for CLI, Godot console, or file output.

use crate::interface::signal::EchoSignal;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log level
#[derive(Clone, Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Ritual,
}

/// Log entry
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
}

/// Logger trait
pub trait EchoLogger {
    fn log(&self, entry: LogEntry);
}

/// Default stdout logger
pub struct StdoutLogger;

impl EchoLogger for StdoutLogger {
    fn log(&self, entry: LogEntry) {
        println!("[{:?}] [{}] {}", entry.level, entry.timestamp, entry.message);
    }
}

/// Helper to create log entries
pub fn log_message(level: LogLevel, message: &str) -> LogEntry {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    LogEntry {
        timestamp,
        level,
        message: message.to_string(),
    }
}
