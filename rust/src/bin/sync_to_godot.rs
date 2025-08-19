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

    let source = PathBuf::from("target/release").join(dll_name);
    let dest = PathBuf::from("../Aetherion_Engine_Tester/bin").join(dll_name);

    println!("🔄 Copying {:?} → {:?}", source, dest);

    match fs::copy(&source, &dest) {
        Ok(_) => println!("✅ Copied successfully."),
        Err(e) => println!("⚠️ Copy failed: {}", e),
    }
}
