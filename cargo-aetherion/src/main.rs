use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let rust_dir = PathBuf::from(r"C:\zv9\zv9.aetherion\rust");
    let target_triple = "x86_64-pc-windows-msvc"; // keep in one place
    let dll_name = "Aetherion_Engine.dll";

    println!("🔨 Building AetherionEngine…");

    let build_status = Command::new("cargo")
        .args(&[
			"build",
			"--release",
			"--features", "macros",
			"--target", target_triple,
		])

        .current_dir(&rust_dir)
        .status()
        .expect("Failed to run cargo build");

    if !build_status.success() {
        eprintln!("❌ Build failed.");
        std::process::exit(1);
    }

    // Build output path is always relative to rust_dir
    let source = rust_dir
        .join("target")
        .join(target_triple)
        .join("release")
        .join(dll_name);

    let dest_dir = PathBuf::from(
        r"C:\zv9\zv9.aetherion\aetherion_engine_tester\addons\S2O_godot_plugin\bin"
    );
    fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");

    let dest = dest_dir.join(dll_name);

    println!("🔍 Source: {:?}", source);
    println!("📁 Destination: {:?}", dest);

    if source.exists() {
        fs::copy(&source, &dest).expect("Failed to copy DLL");
        println!("✅ Sync complete");
    } else {
        eprintln!("⚠️ DLL not found at {:?}", source);
        std::process::exit(1);
    }
}
