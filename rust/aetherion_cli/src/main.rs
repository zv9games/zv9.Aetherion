// --- MODULES ---
// Core CLI components. These files must be defined in the src/ directory.
mod cli_util_actions;	// Menu action functions (e.g., run_tests, print_loc_to_file)
mod cli_util_inspect;	// Inspection functions (API surface, module tree)
mod cli_util_menu;		// Menu structs, build_menu, and print_menu logic
mod cli_util_bench;		// Benchmark and conversion functions

// --- EXTERNAL IMPORTS ---
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use crossterm::event::{self, Event, KeyCode};

// Tracing (logging) imports.
use tracing::{info, error};
use tracing_subscriber::{self, filter::LevelFilter, prelude::*}; // <-- CLEANED (was 'prelude::*;')

// --- INTERNAL IMPORTS ---
use crate::cli_util_menu::{build_menu, print_menu};
use aetherion_engine_ffi::aetherion_initialize_engine; // FFI initialization call

/// 🖐️ Pauses the console until the Enter key is pressed.
fn wait_for_enter() {
	println!("\nPress Enter to return to menu...");
	let _ = io::stdin().read_line(&mut String::new());
}

/// Initializes the console logging system and attempts to boot the Rust core FFI.
fn init_logging_and_engine() {
	// Setup: Simple console logger for the CLI environment.
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::fmt::layer()
				.with_writer(io::stdout) // Direct output to stdout
				.with_filter(LevelFilter::INFO),
		)
		.init();

	info!("AetherionBinary: Interactive CLI initializing.");

	// Call the FFI initialization to boot the engine core.
	if aetherion_initialize_engine() {
		info!("Engine FFI core initialized.");
	} else {
		error!("Failed to initialize Engine FFI core.");
	}
}

fn main() {
	// 🧠 Startup Sequence
	init_logging_and_engine();


	println!(
    r#"
           (__)
           (oo)
     /-------\/
    / |     ||
  * ||----|| // <-- CLEANED (was ' / |     ||' and '*  ||----||')
     ~~    ~~
Aetherion Engine Console Initialized
"#
);


	// 🧭 Menu Initialization
	let menu = build_menu();
	let mut last_keys = HashSet::new(); // Used to prevent rapid-fire key processing

	// 🔁 Main Interaction Loop
	loop {
		print_menu(&menu);
		info!("Console: Awaiting menu selection...");
		print!("> ");
		io::stdout().flush().unwrap();

		// Event polling loop for keypress
		loop {
			if event::poll(Duration::from_millis(500)).unwrap() {
				if let Event::Key(key_event) = event::read().unwrap() {
					if let KeyCode::Char(c) = key_event.code {
						// Only process a key once per poll cycle
						if last_keys.insert(c) { // <-- CLEANED (was 'if last_keys.insert(c) { ')
							if let Some(item) = menu.iter().find(|m| m.key == c) {
								info!("Menu: Selected: {}", item.label);
								println!("\n[{}] {}\n", c, item.label);
								(item.action)();

								// Check for exit command
								if c == '9' {
									info!("Exit: Engine shutdown complete.");
									return;
								}

								wait_for_enter();
								break;	
							}
						}
					}
				}
			} else {
				// Clear keys if no event was found, ready for next input
				last_keys.clear();
			}

			thread::sleep(Duration::from_millis(10));
		}
	}
}