// aetherion_cli/src/cli_util_actions.rs

use std::process::Command;
// FIX 1: Replace log import with tracing import.
use tracing::{info, warn, error};

/// 🚀 Runs the full Rust test suite via Cargo
pub fn run_cargo_tests() {
	println!("🚀 Running full cargo test suite (Placeholder)...");

	let status = Command::new("cargo")
        // Whitespace cleaned here
		.args(&["test", "--", "--nocapture"])
		.status()
		.expect("Failed to run cargo test");

    // Whitespace cleaned here
	if status.success() {
		info!("✅ All tests passed.");
	} else {
		// FIX 2: Replace log::error! with tracing::error!
		error!("❌ Some tests failed.");
	}
}

/// ⚠️ Placeholder for starting the Aetherion core runtime (Conductor)
pub fn start_aetherion_runtime() {
	// FIX 3: Replace log::warn! with tracing::warn!
	warn!("⚠️ Placeholder: Starting Aetherion Runtime...");
	// NOTE: This will be replaced by a call like aetherion_core::core::start(DummyDelivery::new())
	// For now, it's just a log message.
	info!("Runtime placeholder started. Execution complete.");
}

/// 🎮 Placeholder to launch headless Godot
pub fn launch_headless_godot() {
	// FIX 4: Replace log::warn! with tracing::warn!
	warn!("🎮 Placeholder: Attempting to launch headless Godot...");
	// NOTE: Replace paths with your actual environment variables later.
	let godot_path = "C:/ZV9/Godot/Godot.exe";	

	// Simplified status check for the placeholder
	match Command::new(godot_path).arg("--version").status() {
		Ok(status) if status.success() => info!("🚀 Headless Godot launch command ready (path check OK)."),
		// FIX 5: Replace log::error! with tracing::error!
		_ => error!("❌ Godot executable not found or command failed. Check path: {}", godot_path),
	}
}