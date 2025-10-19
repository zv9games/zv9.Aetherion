// aetherion_cli/src/cli_util_inspect.rs
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
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
		"aetherion_cli/src",
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
                                // Fallback to full path if strip_prefix fails (shouldn't happen here)
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


/// 🧪 Scans for GDScript-callable Rust methods exposed via #[func]
pub fn print_godot_api_surface() {
	println!("🧪 API scan triggered (targeting aetherion_godot/src/lib.rs)...");

    let godot_lib_path: PathBuf = PathBuf::from("aetherion_godot/src/lib.rs");
    
    // Regex to find the function signature line that follows #[func].
    // Captures: 1=method_name, 2=arguments (including parentheses), 3=return_type
    // Example target: pub fn generate_chunk_sync(&mut self, x: i32, y: i32) -> bool {
    let fn_signature_regex = Regex::new(
        r"^\s*pub\s+fn\s+(\w+)\s*(\([^\{]*)\s*(?:->\s*([^\{]*))?\s*\{"
    ).unwrap();

    // Regex to find the #[func] marker line.
    let func_marker_regex = Regex::new(r"^\s*#\[func\]\s*$").unwrap();

    let mut api_methods: Vec<(String, String, String)> = Vec::new();
    let mut func_line_pending = false;

    match fs::read_to_string(&godot_lib_path) {
        Ok(contents) => {
            info!("Successfully read {}", godot_lib_path.display());
            
            for line in contents.lines() {
                // 1. Check for the #[func] marker
                if func_marker_regex.is_match(line) {
                    func_line_pending = true;
                    continue;
                }

                // 2. If marker was found, check for the function signature on the next line
                if func_line_pending {
                    if let Some(captures) = fn_signature_regex.captures(line) {
                        
                        let method_name = captures.get(1).map(|m| m.as_str()).unwrap_or("unknown_method").to_string();
                        
                        // Argument capture: remove surrounding parentheses and trim whitespace
                        let args = captures.get(2)
                            .map(|m| m.as_str().trim_start_matches('(').trim_end_matches(')').trim().to_string())
                            .unwrap_or_default();
                        
                        // Return type capture: trim whitespace, default to "()"
                        let return_type = captures.get(3).map_or("()".to_string(), |m| m.as_str().trim().to_string());
                        
                        api_methods.push((method_name, args, return_type));
                        
                        // Reset the flag after processing the function signature
                        func_line_pending = false;
                    } else {
                         // Reset flag if we were expecting a function but found a blank or irrelevant line
                         // (This handles blank lines between the macro and the fn signature)
                         if !line.trim().is_empty() {
                            // If a non-blank line was found that didn't match the signature, 
                            // assume the signature was missed and reset.
                            func_line_pending = false;
                         }
                    }
                }
            }
        },
        Err(e) => {
            error!("Failed to read file {}: {}", godot_lib_path.display(), e);
            println!("\n❌ API Scan Failed. Could not read source file.");
            return;
        }
    }

	println!("\n--- 🎮 AetherionEngine Godot API Surface ---");
    if api_methods.is_empty() {
        warn!("No #[func] methods found in aetherion_godot/src/lib.rs.");
    } else {
        println!("Registered {} callable methods:", api_methods.len());
        for (name, args, return_type) in &api_methods {
            // Display as: MethodName(arguments) -> ReturnType
            println!("  ✅ func {}({}) -> {}", name, args, return_type);
        }
    }
	println!("-------------------------------------------\n");

    info!("API scan complete: {} methods detected.", api_methods.len());
}