//! Aetherion Dev Console — The Wanderer’s Compass
//!
//! Interactive menu for validation, tooling, and runtime control.
//! Press a key. Shape the world.

use tracing::warn;

use crate::cli_util_actions::{
    run_cargo_tests,
    run_priority_1_tests,
    run_ffi_bridge_validation,
    print_loc_to_file,
    launch_godot_client,
    launch_headless_godot,
    start_signal_inspector,
};

use crate::cli_util_inspect::{print_godot_api_surface, print_module_tree};
use crate::cli_util_bench::{run_bitmask_conversion, run_max_grid_benchmark, test_generation_and_placement_cli};

/// Menu entry: key + label + action.
pub struct MenuItem {
    pub key: char,
    pub label: &'static str,
    pub action: Box<dyn Fn()>,
}

/// Constructs the full interactive menu.
pub fn build_menu() -> Vec<MenuItem> {
    vec![
        // ── I. VALIDATION & INSPECTION ─────────────────────────────────────
        MenuItem { key: '0', label: "Run: Cargo Test Suite (Full)", action: Box::new(run_cargo_tests) },
        MenuItem { key: 'C', label: "Validate: Phase 1 Integration", action: Box::new(run_priority_1_tests) },
        MenuItem { key: '9', label: "Validate: FFI Bridge (E2E)", action: Box::new(run_ffi_bridge_validation) },
        MenuItem { key: '1', label: "Inspect: Godot API Surface", action: Box::new(print_godot_api_surface) },
        MenuItem { key: '2', label: "Inspect: Module Tree", action: Box::new(print_module_tree) },

        // ── II. TOOLING & BENCHMARKS ───────────────────────────────────────
        MenuItem { key: 'L', label: "Tool: LOC Report", action: Box::new(print_loc_to_file) },
        MenuItem { key: '6', label: "Convert: PNG → Bitmask", action: Box::new(run_bitmask_conversion) },
        MenuItem { key: '7', label: "Benchmark: Max Grid (100M)", action: Box::new(run_max_grid_benchmark) },
        MenuItem { key: '5', label: "Test: Generation Pipeline", action: Box::new(test_generation_and_placement_cli) },

        // ── III. RUNTIME & DEBUG ───────────────────────────────────────────
        MenuItem { key: '4', label: "Launch: Godot Editor", action: Box::new(launch_godot_client) },
        MenuItem { key: '8', label: "Launch: Headless Godot", action: Box::new(launch_headless_godot) },
        MenuItem { key: 'B', label: "Start: Signal Inspector", action: Box::new(start_signal_inspector) },

        // ── IV. FUTURE & EXIT ──────────────────────────────────────────────
        MenuItem { key: '3', label: "TODO: Trailkeeper Scan", action: Box::new(|| warn!("Trailkeeper not implemented")) },
        MenuItem { key: 'A', label: "TODO: Chunk Hash Export", action: Box::new(|| warn!("Chunk hashing not implemented")) },
        MenuItem { key: 'X', label: "EXIT CONSOLE", action: Box::new(|| {}) },
    ]
}

/// Renders the menu with clean formatting.
pub fn print_menu(menu: &[MenuItem]) {
    println!("\n Aetherion Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nSelect an option...\n");
}