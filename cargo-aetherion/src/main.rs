use std::{fs, path::PathBuf, process::Command};

fn main() {
    let rust_dir = PathBuf::from(r"C:\zv9\zv9.aetherion\rust");
    let target = "x86_64-pc-windows-msvc";
    let dll = "Aetherion_Engine.dll";

    let status = Command::new("cargo")
        .args(["+nightly", "build", "--release", "--target", target, "--lib"])
        .current_dir(&rust_dir)
        .status()
        .expect("Build failed");

    if !status.success() {
        eprintln!("❌ Build failed");
        std::process::exit(1);
    }

    let src = rust_dir.join("target").join(target).join("release").join(dll);
    if !src.exists() {
        eprintln!("⚠️ DLL not found: {:?}", src);
        std::process::exit(1);
    }

    let dst = PathBuf::from(r"C:\zv9\zv9.aetherion\aetherion_engine_tester\addons\S2O_godot_plugin\bin").join(dll);
    fs::create_dir_all(dst.parent().unwrap()).expect("Failed to create bin dir");
    fs::copy(&src, &dst).expect("Failed to copy DLL");

    println!("✅ Build and copy complete");
}
