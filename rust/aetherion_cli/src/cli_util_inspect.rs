//! Aetherion Inspector — The Dev’s X-Ray Vision
//!
//! Module tree scanning + Godot API surface extraction.
//! For the hopeless wanderers who debug with poetry.

use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use tracing::{info, warn, error};
use std::thread;
use std::time::Duration;

// ── Module Tree Scanner ──────────────────────────────────────────────────────

/// Prints a beautiful tree of all Rust modules in the workspace.
pub fn print_module_tree() {
    println!("\n════════════════════════════════════════════════════════════════════════");
    println!("| RUST WORKSPACE MODULE TREE (Scanning...)                           |");
    println!("════════════════════════════════════════════════════════════════════════");

    let crates = [
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

    for crate_dir in crates {
        let path = PathBuf::from(crate_dir);
        println!("\nCrate: {}", crate_dir);

        if !path.exists() {
            warn!("Path missing: {}", crate_dir);
            continue;
        }

        for entry in WalkDir::new(&path).min_depth(1) {
            let Ok(e) = entry else { continue; };
            let p = e.path();

            if !p.is_file() || p.extension() != Some("rs".as_ref()) {
                continue;
            }

            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let prefix = if name == "lib.rs" || name == "main.rs" {
                "├── [CORE] "
            } else {
                "│   └── "
            };

            let rel = p.strip_prefix(&path).unwrap_or(p);
            println!("{} {}", prefix, rel.display());
        }
    }

    println!("════════════════════════════════════════════════════════════════════════\n");
}

// ── Godot API Surface Scanner ───────────────────────────────────────────────

/// Scans `aetherion_godot/src/lib.rs` for `#[func]`-exposed methods.
pub fn print_godot_api_surface() {
    println!("API scan: aetherion_godot/src/lib.rs...");

    let path = Path::new("aetherion_godot/src/lib.rs");
    let Ok(content) = fs::read_to_string(path) else {
        error!("Failed to read Godot lib file: {}", path.display());
        println!("\nAPI Scan FAILED.\n");
        return;
    };

    let marker = Regex::new(r"^\s*#\[\s*func\s*\]\s*$").unwrap();
    let sig = Regex::new(r"^\s*pub\s+fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*([^\s{]+))?\s*\{").unwrap();

    let mut methods = Vec::new();
    let mut pending = false;

    for line in content.lines() {
        if marker.is_match(line) {
            pending = true;
            continue;
        }

        if pending {
            if let Some(caps) = sig.captures(line) {
                let name = caps[1].to_string();
                let args = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("").to_string();
                let ret = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("()").to_string();
                methods.push((name, args, ret));
                pending = false;
            } else if !line.trim().is_empty() {
                pending = false; // Non-matching line after #[func]
            }
        }
    }

    println!("\n--- AetherionEngine Godot API Surface ---");
    if methods.is_empty() {
        warn!("No #[func] methods found. Is the Godot binding active?");
    } else {
        println!("Registered {} callable methods:", methods.len());
        for (name, args, ret) in &methods {
            println!("  func {}({}) -> {}", name, args, ret);
        }
    }
    println!("-------------------------------------------\n");

    info!("API scan complete: {} methods exposed.", methods.len());
    thread::sleep(Duration::from_secs(2));
}