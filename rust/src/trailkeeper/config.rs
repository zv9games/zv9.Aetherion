/// ✅ Suggestions for trailkeeper/config_watch.rs

// 🔧 Add error handling via `Result`:
//     - Return `Result<(), ConfigWatchError>` instead of panicking with `expect()`
//     - Enables graceful failure and integration with higher-level systems

// 🧩 Add support for multiple config files:
//     - Accept `&[&str]` or `Vec<PathBuf>` to monitor multiple configs
     - Useful for modular setups or plugin architectures

// 🚦 Improve hash storage strategy:
//     - Store hashes in a structured format (e.g. JSON or TOML)
//     - Enables tracking multiple files and avoids collisions

// 📚 Document hashing behavior:
//     - Clarify that SHA-256 is used and why (e.g. consistency, security)
//     - Note that changes are detected by content, not timestamp

// 🧪 Add unit tests with mock config files:
//     - Validate hash comparison, log entry creation, and file write behavior

// 🧼 Optional: Add verbosity or logging:
//     - Log when config is unchanged for traceability
     - Could emit `LogStatus::Info` or skip silently

// 🚀 Future: Add live reload or callback hook:
//     - e.g. `fn on_config_change(callback: impl FnOnce())`
//     - Enables reactive behavior or dynamic reconfiguration

// 🧠 Consider exposing config state snapshot:
//     - `fn current_config_hash() -> String`
//     - Useful for diagnostics, UI display, or syncing


use crate::trailkeeper::entry::{LogEntry, EventType, LogStatus};
use crate::trailkeeper::collector::Trailkeeper;
use chrono::Utc;
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};

static CONFIG_PATH: &str = "config.toml";
static HASH_STORE: &str = ".trailkeeper_config_hash";

pub fn check_config_change() {
    if !Path::new(CONFIG_PATH).exists() {
        return;
    }

    let config_data = fs::read(CONFIG_PATH).expect("Failed to read config");
    let mut hasher = Sha256::new();
    hasher.update(&config_data);
    let current_hash = format!("{:x}", hasher.finalize());

    let previous_hash = fs::read_to_string(HASH_STORE).unwrap_or_default();

    if current_hash != previous_hash {
        Trailkeeper::record(LogEntry {
            event_type: EventType::ConfigChange,
            timestamp: Utc::now(),
            actor: "system".to_string(),
            description: "Config file changed".to_string(),
            affected_components: vec![CONFIG_PATH.to_string()],
            status: LogStatus::Detected,
        });

        fs::write(HASH_STORE, current_hash).expect("Failed to write hash");
    }
}
