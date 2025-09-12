/// ✅ Suggestions for trailkeeper/git_diff.rs

// 🔧 Add support for staged or unstaged diffs:
//     - e.g. `"git diff --cached"` or `"git diff HEAD"`
//     - Enables broader change tracking across commit boundaries

// 🧩 Add filtering or file type awareness:
//     - Only log changes to `.rs`, `.toml`, or config files
     - Reduces noise and improves relevance

// 🚦 Improve error handling:
//     - Use `Result` instead of `expect()` for better resilience
     - Log stderr output if `git diff` fails

// 📚 Document assumptions about diff format:
//     - Clarify that this parses unified diff headers and hunks
     - Note that line numbers are approximate and may vary

// 🧪 Add unit tests with mock diff output:
//     - Validate file extraction, line parsing, and log entry creation
     - Ensure robustness against malformed or partial diffs

// 🧼 Optional: Add change type classification:
//     - e.g. `Added`, `Modified`, `Deleted` based on diff markers
     - Could extend `EventType` or add a `change_kind` field

// 🚀 Future: Add support for diff summary or metrics:
//     - Count lines changed, files affected, or diff size
     - Useful for dashboards or commit impact analysis

// 🧠 Consider batching log entries:
//     - Collect all entries and submit as a group to `Trailkeeper`
     - Reduces overhead and enables grouped reporting


use crate::trailkeeper::entry::{LogEntry, EventType, LogStatus};
use crate::trailkeeper::collector::Trailkeeper;
use chrono::Utc;
use std::process::Command;

pub fn scan_git_diff() {
    let output = Command::new("git")
        .args(&["diff", "--unified=0", "HEAD~1"])
        .output()
        .expect("Failed to run git diff");

    let diff = String::from_utf8_lossy(&output.stdout);
    let mut current_file: Option<String> = None;

    for line in diff.lines() {
        if line.starts_with("diff --git") {
            // Extract file path from the diff header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let file_path = parts[2].trim_start_matches("b/");
                current_file = Some(file_path.to_string());
            }
        } else if line.starts_with("@@") {
            // Extract line number from hunk header
            if let Some(file) = &current_file {
                let hunk_parts: Vec<&str> = line.split_whitespace().collect();
                let line_info = hunk_parts.get(2).unwrap_or(&"");
                let line_number = line_info
                    .trim_start_matches('+')
                    .split(',')
                    .next()
                    .unwrap_or("?");

                Trailkeeper::record(LogEntry {
                    event_type: EventType::GitDiff,
                    timestamp: Utc::now(),
                    actor: "system".to_string(),
                    description: format!("Modified {} at line {}", file, line_number),
                    affected_components: vec![file.clone()],
                    status: LogStatus::Detected,
                });
            }
        }
    }
}
