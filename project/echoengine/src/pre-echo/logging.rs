use godot::prelude::*;

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Info => godot_print!("{}", message),
        LogLevel::Warn => godot_warn!("{}", message),
        LogLevel::Error => godot_error!("{}", message),
    }
}