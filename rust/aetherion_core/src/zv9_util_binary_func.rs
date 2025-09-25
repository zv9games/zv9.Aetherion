// c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary_func.rs

#[allow(unused_imports)]
use crate::trailkeeper::entry::EventType;

use crate::trailkeeper::{
    collector::Trailkeeper,
    config::check_config_change,
    scan::scan_git_diff,
    entry::LogEntry,
};


use crate::{Conductor, ProcCommand, GodotSync};

use std::process::Command;
use std::io::{self, Write};

/// 🚀 Runs the full Rust test suite via Cargo
pub fn run_cargo_tests() {
    println!("🚀 Running full cargo test suite...\n");

    let status = Command::new("cargo")
        .args(&["test", "--", "--nocapture"])
        .status()
        .expect("Failed to run cargo test");

    if status.success() {
        println!("✅ All tests passed.");
    } else {
        println!("❌ Some tests failed.");
    }
}

/// 📋 Inspects the procedural command queue
pub fn inspect_pending_queue() {
    let mut conductor = Conductor::new(GodotSync::init());
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}

/// 🔍 Runs a Trailkeeper scan for changes and config diffs
pub fn run_trailkeeper_scan() {
    println!("🔍 Running Trailkeeper scan...\n");

    scan_git_diff();
    check_config_change();

    for log in Trailkeeper::all() {
        println!("{:?}", log);
    }

    println!("\n✅ Trailkeeper scan complete.\n");
}

/// 📜 Interactive viewer for Trailkeeper logs
pub fn view_trailkeeper_logs() {
    println!("\n📜 Trailkeeper Log Registry:\n");

    let logs = Trailkeeper::all();
    if logs.is_empty() {
        println!("(No logs recorded yet.)");
        return;
    }

    let stdin = io::stdin();
    let mut buffer = String::new();

    for (i, log) in logs.iter().enumerate() {
        print_log_entry(i + 1, log);
        print!("Press Enter to continue, or type 9 to quit: ");
        io::stdout().flush().unwrap();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        if buffer.trim() == "9" {
            println!("\n🚪 Exiting log viewer...\n");
            break;
        }
    }

    println!("\n✅ Log inspection complete.\n");
}

/// 🧾 Prints a formatted Trailkeeper log entry
pub fn print_log_entry(index: usize, log: &LogEntry) {
    println!("──────────────────────────────────────────────");
    println!("📄 Entry #{}", index);
    println!("🕒 Timestamp: {}", log.timestamp.to_rfc3339());
    println!("🧠 Event Type: {:?}", log.event_type);
    println!("👤 Actor: {}", log.actor);
    println!("📝 Description: {}", log.description);
    println!("📦 Components: {:?}", log.affected_components);
    println!("⚠️ Status: {:?}", log.status);
    println!("──────────────────────────────────────────────");
}


// the end
