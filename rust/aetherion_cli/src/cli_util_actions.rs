// aetherion_cli/src/cli_util_actions.rs

use std::process::Command;
use tracing::{info, warn, error};

// PHASE 2 TRANSITION: Import the new runtime entry point
use aetherion_generate::start_runtime_placeholder;

/// 🚀 Runs the full Rust test suite via Cargo
pub fn run_cargo_tests() {
	println!("🚀 Running full cargo test suite (Placeholder)...");

	let status = Command::new("cargo")
		.args(&["test", "--", "--nocapture"])
		.status()
		.expect("Failed to run cargo test");

	if status.success() {
		info!("✅ All tests passed.");
	} else {
		error!("❌ Some tests failed.");
	}
}

/// 🚀 Starts the Aetherion core runtime (Conductor) structure check.
///
/// This executes the Phase 2 immediate priority: Conductor initialization and graceful shutdown.
pub fn start_aetherion_runtime() {
    // FIX: Execute the Conductor structural test.
    start_runtime_placeholder();
    info!("🚀 Runtime structural test complete (Conductor booted and shut down gracefully).");
}

/// 🎮 Placeholder to launch headless Godot
pub fn launch_headless_godot() {
	warn!("🎮 Placeholder: Attempting to launch headless Godot...");
	// NOTE: Replace paths with your actual environment variables later.
	let godot_path = "C:/zv9/zv9.aetherion/rust/godot.windows.editor.x86_64.exe";	

	// Simplified status check for the placeholder
	match Command::new(godot_path).arg("--version").status() {
		Ok(status) if status.success() => info!("🚀 Headless Godot launch command ready (path check OK)."),
		_ => error!("❌ Godot executable not found or command failed. Check path: {}", godot_path),
	}
}

/// Runs only the unit tests defined in the Phase 1 Foundation Layer packages.
///
/// This serves as the final integration check before progressing to Phase 2.
pub fn run_priority_1_tests() {
    info!("Running Phase 1 Foundation Layer (P1) test suite...");

    // Target packages: aetherion_shared, aetherion_math, aetherion_sync
    // Using --all-targets to include integration tests if they exist.
    let result = Command::new("cargo")
        .arg("test")
        .arg("--package")
        .arg("aetherion_shared")
        .arg("--package")
        .arg("aetherion_math")
        .arg("--package")
        .arg("aetherion_sync")
        .arg("--all-targets")
        .status();

    match result {
        Ok(status) if status.success() => {
            info!("✅ Phase 1 Validation Complete: All foundation tests passed successfully.");
        }
        _ => {
            error!("❌ Phase 1 Validation Failed. Check the errors above.");
        }
    }
}