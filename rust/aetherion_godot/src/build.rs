//! Aetherion Godot Build Script — The Silent Deployer
//!
//! Automatically copies the compiled GDExtension DLL from `target/`
//! into the Godot test project on every build.
//! For the hopeless wanderers who hate manual file shuffling.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap();
    let dll_name = "aetherion_engine.dll";

    // Source: target/<profile>/aetherion_engine.dll
    let src = manifest_dir
        .parent().unwrap()           // rust/
        .join("target")
        .join(&profile)
        .join(dll_name);

    // Destination: ../../aetherion_engine_tester/aetherion_engine.dll
    let dst = manifest_dir
        .parent().unwrap()           // rust/
        .parent().unwrap()           // zv9.aetherion/
        .join("aetherion_engine_tester")
        .join(dll_name);

    // Windows: cmd /C copy /Y (force overwrite)
    let status = Command::new("cmd")
        .args(["/C", "copy", "/Y"])
        .arg(&src)
        .arg(&dst)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:warning=DEPLOYED: {} → {}", src.display(), dst.display());
        }
        _ => {
            println!("cargo:warning=DEPLOY FAILED!");
            println!("cargo:warning=  From: {}", src.display());
            println!("cargo:warning=  To:   {}", dst.display());
            println!("cargo:warning=  Tip: Check file locks, permissions, or Godot running.");
        }
    }
}