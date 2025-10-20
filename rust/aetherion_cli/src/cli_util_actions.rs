//! Core action implementations for the Aetherion Dev Console CLI menu.

use std::process::Command;
use tracing::{info, warn, error};

// NEW IMPORTS for Signal Inspector / Concurrency
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use std::io::{self, Write}; // io::Write for flush()
use ctrlc; // Used for graceful shutdown handling

// PHASE 2 TRANSITION: Import the Conductor types
// FIX: Removed unused import 'ConductorState'
use aetherion_generate::conductor::{Conductor, ConductorStatus}; 

// --- CORE CLI ACTIONS ---

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

/// 🚀 Starts the Aetherion core runtime (Conductor) structural test.
///
/// This serves as a quick check that the Conductor can initialize and shut down cleanly.
pub fn start_aetherion_runtime() {
    warn!("🚀 Running Conductor structural test...");
    match Conductor::new() {
        Ok((conductor, _state)) => {
            info!("✅ Conductor initialized successfully.");
            // FIX: Use the renamed consuming method for full teardown.
            conductor.graceful_teardown(); 
            info!("✅ Conductor shut down gracefully.");
        }
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
        }
    }
}

/// 🎮 Placeholder to launch headless Godot
pub fn launch_headless_godot() {
	warn!("🎮 Placeholder: Attempting to launch headless Godot...");
	let godot_path = "C:/zv9/zv9.aetherion/rust/godot.windows.editor.x86_64.exe";	

	// Simplified status check for the placeholder
	match Command::new(godot_path).arg("--version").status() {
		Ok(status) if status.success() => info!("🚀 Headless Godot launch command ready (path check OK)."),
		_ => error!("❌ Godot executable not found or command failed. Check path: {}", godot_path),
	}
}

/// Runs only the unit tests defined in the Phase 1 Foundation Layer packages.
pub fn run_priority_1_tests() {
	info!("Running Phase 1 Foundation Layer (P1) test suite...");

	// Target packages: aetherion_shared, aetherion_math, aetherion_sync
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

// -----------------------------------------------------------------------------
// PHASE 4: SIGNAL INSPECTOR / LIVE FEED
// -----------------------------------------------------------------------------

/// 🔮 Starts the live **Signal Inspector** utility (CLI Menu [B]).
///
/// This function initializes the **Conductor** and begins a real-time inspection loop
/// using the shared `ConductorState`.
pub fn start_signal_inspector() {
    warn!("🔮 Initializing Conductor and starting Signal Inspector (Real-Time Feed)...");

    // 1. Initialize Conductor and retrieve the thread-safe state
    let (conductor, state) = match Conductor::new() {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Failed to initialize Conductor/Runtime: {}", e);
            return;
        }
    };
    
    // Wrap Conductor in Arc<Mutex<Option<>>> for safe, single consumption by the ctrlc handler.
    // We move the Conductor to a dedicated variable that will be consumed on shutdown.
    let conductor_shutdown_safe = Arc::new(Mutex::new(Some(conductor)));
    let shutdown_clone = conductor_shutdown_safe.clone();
    
    // 2. Setup atomic flag for graceful exit via Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r_for_handler = running.clone(); // Clone for the handler closure

    // Set a Ctrl-C handler to stop the loop and shut down the Conductor
    if let Err(e) = ctrlc::set_handler(move || {
        // Signal the main loop to stop
        r_for_handler.store(false, Ordering::SeqCst);
        
        // Atomically take the Conductor out of the Mutex and shut it down once.
        if let Some(c) = shutdown_clone.lock().unwrap().take() {
            // FIX: Use the renamed consuming method.
            c.graceful_teardown();
        }
        // Print message directly to console
        let _ = writeln!(io::stdout(), "\nInspector: Shutdown signal received. Waiting for Conductor...");
    }) {
        error!("Could not set Ctrl-C handler: {}", e);
        return;
    }

    info!("Inspector: Press Ctrl-C to stop the live feed and gracefully shut down the Conductor.");

    let mut frame_count: u64 = 0;
    const MVG_BASELINE: u64 = 10_000_000;
    
    // 3. Main Live Feed Loop
    while running.load(Ordering::SeqCst) {
        frame_count += 1;
        
        // --- REAL-TIME DATA POLLING ---
        let status = state.get_status();
        let queue_depth = state.get_queue_depth();
        let active_id = state.get_active_generator_id();

        // Use carriage return (`\r`) to overwrite the current line.
        print!("\r"); 
        print!("🔮 LIVE FEED | Frame: {: >4} | Status: {: <12} | Generator: {: <25} | Queue Depth: ~{: >6} | MVG Baseline: {} tiles/s | Press Ctrl-C to exit.",
            frame_count,
            format!("{:?}", status), 
            active_id,
            queue_depth,
            MVG_BASELINE
        );
        let _ = io::stdout().flush();
        
        // Check for internal shutdown signals (e.g., error in Conductor)
        if status == ConductorStatus::ShuttingDown || status == ConductorStatus::Error {
            // Signal the loop to stop
            running.store(false, Ordering::SeqCst); 
            
            // If the Conductor hasn't already been consumed by ctrlc handler, shut it down here
            if let Some(c) = conductor_shutdown_safe.lock().unwrap().take() {
                // FIX: Use the renamed consuming method.
                c.graceful_teardown();
            }
            break; 
        }

        // Wait 50ms (20 FPS refresh)
        thread::sleep(Duration::from_millis(50));
    }
    
    // 4. Cleanup: Clear the line after exiting the loop
    // Print a newline after the carriage return overwrite to ensure clean console
    let _ = writeln!(io::stdout(), "\r{: <200}", " "); // Overwrite and clear the line
    info!("Inspector shutdown complete. Conductor runtime terminated.");
}

// --- PLACEHOLDER/BENCHMARK ACTIONS ---

// Note: test_generation_and_placement_cli, run_bitmask_conversion, 
// and run_max_grid_benchmark are assumed to be implemented in cli_util_bench.rs