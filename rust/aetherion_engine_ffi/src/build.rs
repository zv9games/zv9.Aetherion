// aetherion_engine_ffi/build.rs

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // --- 1. Define Paths ---

    // The current output directory of the FFI DLL (e.g., target/debug/)
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Determine the Godot project root relative to the CWD (which is aetherion_engine_ffi/)
    // Our workspace root is `C:\ZV9\zv9.aetherion\rust`.
    // The Godot project is `C:\ZV9\zv9.aetherion\aetherion_engine_tester`.
    // Path from CWD to Godot project: ../../aetherion_engine_tester/
    let godot_project_path = PathBuf::from("../../aetherion_engine_tester");
    
    // Determine the profile (debug or release)
    let profile = env::var("PROFILE").unwrap(); 
    let dll_name = "aetherion_engine.dll";

    // --- 2. Calculate Source Path (Where Cargo places the DLL) ---

    // The DLL is usually placed one level up from OUT_DIR, 
    // inside the profile directory (debug/release).
    // Example: target/debug/aetherion_engine.dll
    let mut src_path = out_dir.parent().unwrap().to_path_buf(); // target/
    src_path.push(&profile); // target/debug/
    src_path.push(dll_name); // target/debug/aetherion_engine.dll


    // --- 3. Calculate Destination Path (Where Godot expects the DLL) ---

    // The DLL needs to be placed directly in the Godot project root.
    // Example: C:\ZV9\zv9.aetherion\aetherion_engine_tester\aetherion_engine.dll
    let mut dst_path = godot_project_path;
    dst_path.push(dll_name);


    // --- 4. Copy the DLL ---
    if let Err(e) = fs::copy(&src_path, &dst_path) {
        println!(
            "cargo:warning=Failed to copy DLL from {} to {}: {}",
            src_path.display(),
            dst_path.display(),
            e
        );
        // If the file simply doesn't exist yet, it's not a panic condition
    } else {
        println!(
            "cargo:warning=Successfully copied {} to {}",
            dll_name,
            dst_path.display()
        );
    }
}