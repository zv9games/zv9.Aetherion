use std::fs;
use std::path::PathBuf;

fn main() {
    let dll_name = if cfg!(target_os = "windows") {
        "Aetherion_Engine.dll"
    } else if cfg!(target_os = "linux") {
        "libaetherion.so"
    } else if cfg!(target_os = "macos") {
        "libaetherion.dylib"
    } else {
        panic!("Unsupported OS");
    };

    let source = PathBuf::from(
        r"C:\zv9\zv9.aetherion\rust\target\x86_64-pc-windows-msvc\release"
    ).join(dll_name);

    let dest = PathBuf::from(
        r"C:\zv9\zv9.aetherion\aetherion_engine_tester\addons\S2O_godot_plugin\bin"
    ).join(dll_name);

    println!("🔄 Copying {:?} → {:?}", source, dest);

    if let Err(e) = fs::copy(&source, &dest) {
        eprintln!("⚠️ Copy failed: {}", e);
    } else {
        println!("✅ Copied successfully.");
    }
}
