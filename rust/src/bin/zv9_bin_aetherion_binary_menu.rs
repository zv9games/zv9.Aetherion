//c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary_menu.rs
use super::zv9_bin_aetherion_binary_func::{
    run_cargo_tests,
    print_godot_api_surface,
    inspect_pending_queue,
    run_trailkeeper_scan,
    view_trailkeeper_logs,
    print_module_tree,
    run_max_grid_benchmark,
    run_bitmask_conversion,
};
use aetherion_engine::core::runtime::start as start_runtime;

/// Menu item definition
pub struct MenuItem {
    pub key: char,
    pub label: &'static str,
    pub action: fn(),
}

pub fn build_menu() -> Vec<MenuItem> {
    vec![
        MenuItem { key: '0', label: "Run: Cargo Test Suite", action: run_cargo_tests },
        MenuItem { key: '1', label: "Inspect: Godot-Callable API", action: print_godot_api_surface },
        MenuItem { key: '2', label: "Inspect: Pending Queue", action: inspect_pending_queue },
        MenuItem { key: '3', label: "Run: Trailkeeper Scan", action: run_trailkeeper_scan },
        MenuItem { key: '4', label: "View: Trailkeeper Logs", action: view_trailkeeper_logs },
        MenuItem { key: '5', label: "Start: Aetherion Runtime", action: start_runtime },
        MenuItem { key: '6', label: "Benchmark: Max Grid Placement", action: run_max_grid_benchmark },
        MenuItem { key: '7', label: "Perform: Bitmask PNG Conversion", action: run_bitmask_conversion },
        MenuItem { key: '8', label: "Inspect: API Surface", action: print_module_tree },
        MenuItem { key: '9', label: "Exit", action: || {} },
    ]
}

pub fn print_menu(menu: &[MenuItem]) {
    println!("\n🧭 Aetherion Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nPress a number key to select an option...\n");
}
// the end