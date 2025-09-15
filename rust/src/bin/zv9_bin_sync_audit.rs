use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("🔍 Starting Sync Audit...");

    // Step 1: Scan all .rs files for phantom DLL references
    println!("\n📜 Scanning .rs files for 'aetherion.dll'...");
    let rs_files = find_files_with_extension(".", "rs");
    for file in rs_files {
        if let Ok(content) = fs::read_to_string(&file) {
            if content.contains("aetherion.dll") {
                println!("👻 Phantom reference found in: {:?}", file);
            }
        }
    }

    // Step 2: List binaries in target/release
    println!("\n📦 Binaries in target/release:");
    let release_dir = Path::new("target/release");
    if let Ok(entries) = fs::read_dir(release_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && is_binary(&path) {
                println!(" - {:?}", path.file_name().unwrap());
            }
        }
    }

    // Step 3: Optional — print last build time
    println!("\n⏱️ Last build timestamp:");
    if let Ok(_output) = Command::new("cargo")
        .args(&["build", "--release", "--timings"])
        .output()
    {
        println!("(Build triggered for timing audit)");
    }

    println!("\n✅ Sync audit complete.");
}

fn find_files_with_extension(dir: &str, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(find_files_with_extension(path.to_str().unwrap(), ext));
            } else if path.extension().map_or(false, |e| e == ext) {
                files.push(path);
            }
        }
    }
    files
}

fn is_binary(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        name.ends_with(".exe") || name.ends_with(".dll") || name.ends_with(".so") || name.ends_with(".dylib")
    } else {
        false
    }
}
