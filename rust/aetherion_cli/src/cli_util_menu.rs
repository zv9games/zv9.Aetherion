// aetherion_cli/src/cli_util_menu.rs

// FIX: Replace the log import with the tracing import.
use tracing::warn;
// NEW IMPORT: Add the new function to be called from the menu
use crate::cli_util_actions::{
    run_cargo_tests, 
    start_aetherion_runtime, 
    launch_headless_godot, 
    run_priority_1_tests, 
    start_signal_inspector,
    run_ffi_bridge_validation, // <--- 💡 ADDED THE NEW FFI VALIDATION FUNCTION
}; 
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
		
		// 🎮 Engine Integration (Phase 7 Focus)
		MenuItem { 
            key: '8', 
            label: "🎮 Launch: Headless Godot (External)", 
            action: Box::new(launch_headless_godot) 
        },
        // 🔥 IMMEDIATE PRIORITY FOR V7.0 FFI_FINALIZATION
		MenuItem { 
            key: '9', 
            label: "🔥 Validate: FFI Bridge Data Transfer (Godot E2E)", // Updated label to reflect E2E test
            action: Box::new(run_ffi_bridge_validation) // <--- NEW ACTION
        },
		
		// 🚪 Exit
		MenuItem { key: 'E', label: "✅ Exit", action: Box::new(|| {}) }, // Using 'E' for Exit to free '9'
		
		// 🔮 Future Expansion / TODOs
		MenuItem { key: 'A', label: "🔮 TODO: Export Chunk Hashes for Streaming", action: Box::new(|| warn!("TODO: Chunk hashing not yet implemented.")) },
		// 🔥 PHASE 4: Signal Inspector is now a callable action
		MenuItem { key: 'B', label: "🔮 Start: Signal Inspector / Live Feed", action: Box::new(start_signal_inspector) }, 
		// NEW ITEM: Dedicated Phase 1 validation test
		MenuItem { key: 'C', label: "✅ Validate: Phase 1 Final Integration Check", action: Box::new(run_priority_1_tests) },
        // We removed the old exit on key '9' and used it for the new, critical FFI test.
	]
}

/// 🖥 Prints the menu to the console
pub fn print_menu(menu: &[MenuItem]) {
	
	println!("\n🧭 Aetherion Engine Dev Console\n");
	for item in menu {
		println!("[{}] {}", item.key, item.label);
	}
	println!("\nSelect an option by pressing its number key or letter key...\n");
}