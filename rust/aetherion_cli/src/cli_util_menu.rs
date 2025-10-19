// aetherion_cli/src/cli_util_menu.rs

// FIX: Replace the log import with the tracing import.
use tracing::warn;
// NEW IMPORT: Add the new function to be called from the menu
use crate::cli_util_actions::{run_cargo_tests, start_aetherion_runtime, launch_headless_godot, run_priority_1_tests};
use crate::cli_util_inspect::{print_godot_api_surface, print_module_tree};
use crate::cli_util_bench::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};

/// 🧩 Menu item definition
pub struct MenuItem {
	pub key: char,
	pub label: &'static str,
	pub action: Box<dyn Fn()>,
}

/// 🧭 Builds the interactive dev console menu
pub fn build_menu() -> Vec<MenuItem> {
	vec![
		// ✅ Core Actions & Inspection
		MenuItem { key: '0', label: "✅ Run: Cargo Test Suite", action: Box::new(run_cargo_tests) },
		MenuItem { key: '1', label: "✅ Inspect: Godot-Callable API Surface", action: Box::new(print_godot_api_surface) },
		MenuItem { key: '2', label: "✅ Inspect: Rust Module Tree", action: Box::new(print_module_tree) },
		MenuItem { key: '3', label: "⚠️ Run: Trailkeeper Scan (TODO)", action: Box::new(|| warn!("TODO: Trailkeeper scan not yet implemented.")) },
		
		// 🚀 Runtime & Benchmarks
		MenuItem { key: '4', label: "🚀 Start: Aetherion Runtime (Placeholder)", action: Box::new(start_aetherion_runtime) },
		MenuItem { key: '5', label: "🧪 Test: Generation & Placement CLI", action: Box::new(test_generation_and_placement_cli) },
		MenuItem { key: '6', label: "✅ Perform: Bitmask PNG Conversion", action: Box::new(run_bitmask_conversion) },
		MenuItem { key: '7', label: "🧪 Benchmark: Max Grid Placement", action: Box::new(run_max_grid_benchmark) },
		
		// 🎮 Engine Integration
		MenuItem { key: '8', label: "🎮 Launch: Headless Godot (External)", action: Box::new(launch_headless_godot) },
		
		// 🚪 Exit
		MenuItem { key: '9', label: "✅ Exit", action: Box::new(|| {}) },

		// 🔮 Future Expansion / TODOs
		MenuItem { key: 'A', label: "🔮 TODO: Export Chunk Hashes for Streaming", action: Box::new(|| warn!("TODO: Chunk hashing not yet implemented.")) },
		MenuItem { key: 'B', label: "🔮 TODO: Signal Inspector / Live Feed", action: Box::new(|| warn!("TODO: Signal inspector not yet implemented.")) },
		// NEW ITEM: Dedicated Phase 1 validation test
		MenuItem { key: 'C', label: "✅ Validate: Phase 1 Final Integration Check", action: Box::new(run_priority_1_tests) },
	]
}

/// 🖥 Prints the menu to the console
pub fn print_menu(menu: &[MenuItem]) {
  
	println!("\n🧭 Aetherion Engine Dev Console\n");
	for item in menu {
		println!("[{}] {}", item.key, item.label);
	}
	println!("\nSelect an option by pressing its number key...\n");
}