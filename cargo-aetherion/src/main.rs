use std::fs;
use std::path::PathBuf;

fn main() {
    println!("🔨 Building AetherionEngine...");
    let build_status = std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("../rust")
        .status()
        .expect("Failed to run cargo build");

    if !build_status.success() {
        eprintln!("❌ Build failed.");
        std::process::exit(1);
    }

    println!("📦 Syncing DLL to Godot bin folder...");

    let source = PathBuf::from("../rust/target/release/Aetherion_Engine.dll");
    let dest_dir = PathBuf::from("../aetherion_engine_tester");

    // Ensure destination directory exists
    if !dest_dir.exists() {
        println!("🧱 Destination folder missing. Creating...");
        fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
    }

    let dest = dest_dir.join("Aetherion_Engine.dll");

    println!("🔍 Source: {:?}", source);
    println!("📁 Destination: {:?}", dest);

    if source.exists() {
        fs::copy(&source, &dest).expect("Failed to copy DLL");
        println!("✅ Sync complete: {:?}", dest);
    } else {
        eprintln!("⚠️ DLL not found: {:?}", source);
    }
}
