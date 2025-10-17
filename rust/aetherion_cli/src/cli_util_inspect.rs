// aetherion_cli/src/cli_util_inspect.rs

use walkdir::WalkDir;
use std::path::Path;
use regex::Regex;
use std::fs;
use log::{info, warn};

/// 📦 Prints a tree of Rust modules across all workspace crates
pub fn print_module_tree() {
    println!("\n📦 Scanning for Rust modules across workspace (using WalkDir)...");

    // Define paths relative to the workspace root for scanning
    let crate_dirs = [
        "../aetherion_core/src",
        "../aetherion_engine/src",
        "src", // aetherion_cli/src
    ];

    for crate_dir in crate_dirs {
        // NOTE: Path resolution might need adjustment based on how you run the binary.
        // For cargo run from the root, this is often tricky.
        println!("🔍 Crate: {} (Placeholder check)", crate_dir);
        // Placeholder implementation for now:
        if Path::new(crate_dir).exists() {
            info!("Path exists. (Full tree generation skipped for brevity)");
        } else {
            warn!("Path does not exist: {}", crate_dir);
        }
    }
    println!("\n✅ Module scan complete.\n");
}

/// 🧪 Scans for GDScript-callable Rust methods exposed via #[func] (Placeholder)
pub fn print_godot_api_surface() {
    println!("🧪 API scan triggered (using Regex on source files)...");

    // Placeholder check using Regex dependency
    let class_marker = Regex::new(r"#\[\s*derive\s*\(\s*GodotClass\s*\)\s*]").unwrap();
    
    // NOTE: Actual implementation requires a full scan loop over all Rust files.
    // We only perform a placeholder check here.

    if class_marker.is_match("#[derive(GodotClass)]") {
        info!("Regex check passed: GodotClass attribute recognized.");
    } else {
        log::error!("Regex check failed!");
    }
    
    println!("\n📊 Summary: Placeholder check complete.");
    println!("✅ GDScript-callable methods printed (Placeholder only).\n");
}