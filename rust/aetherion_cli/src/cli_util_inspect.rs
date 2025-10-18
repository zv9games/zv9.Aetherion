// aetherion_cli/src/cli_util_inspect.rs
use walkdir::WalkDir;
use std::path::Path;
use regex::Regex; 
use tracing::{info, warn, error};


/// 📦 Prints a tree of Rust modules across all workspace crates
pub fn print_module_tree() {
	println!("\n📦 Scanning for Rust modules across workspace (using WalkDir)...");

	// Corrected Crate list based on manifest.rs
	let crate_dirs = [
		"aetherion_cache/src",
		"aetherion_engine_ffi/src",
		"aetherion_generate/src",
		"aetherion_godot/src",
		"aetherion_math/src",
		"aetherion_shared/src",
		"aetherion_sync/src",
		"aetherion_tools/src",
		"aetherion_cli/src", // aetherion_cli/src
	];

	for crate_dir in crate_dirs {
        let crate_path = Path::new(crate_dir);
		println!("\n🔍 Crate: {}", crate_dir);

		if crate_path.exists() && crate_path.is_dir() {
            // Use WalkDir to traverse the directory
			for entry in WalkDir::new(crate_path) {
				match entry {
					Ok(e) => {
                        let path = e.path();
                        // Only process files that end with `.rs`
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                            // Print a simple tree-like structure
                            let prefix = if path.file_name().map_or(false, |name| name == "lib.rs" || name == "main.rs") {
                                // Important files are highlighted
                                "├── [CORE] "
                            } else {
                                "│   └── "
                            };
                            
                            // Get the path relative to the crate's src folder
                            if let Ok(relative_path) = path.strip_prefix(crate_path) {
                                println!("{} {}", prefix, relative_path.display());
                            } else {
                                println!("{} {}", prefix, path.display());
                            }
                        }
					},
					Err(e) => warn!("Error walking directory {}: {}", crate_dir, e),
				}
			}
		} else {
			warn!("Path does not exist or is not a directory: {}", crate_dir);
		}
	}
	println!("\n✅ Module scan complete.\n");
}

// ... (print_godot_api_surface() remains the same for now)

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
		// FIX 2: Replace log::error! with tracing::error!
		error!("Regex check failed!");
	}
	
	println!("\n📊 Summary: Placeholder check complete.");
	println!("✅ GDScript-callable methods printed (Placeholder only).\n");
}