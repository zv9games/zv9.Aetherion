// src/bin/audit_macro.rs

use std::env;
use std::fs;
use std::path::PathBuf;

/// 🐾 Finds the source file containing `APIBot` and checks for macro annotations.
fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src = root.join("src").join("api_bot.rs");

    let contents = fs::read_to_string(&src).expect("Failed to read api_bot.rs");

    let has_godot_class = contents.contains("#[derive(GodotClass)]");
    let has_class_attr = contents.contains("#[class(");
    let has_godot_api = contents.contains("#[godot_api]");
    let is_pub = contents.contains("pub struct APIBot");

    println!("🔍 Macro Audit for `APIBot`");
    println!("📄 File: {}", src.display());
    println!("✅ #[derive(GodotClass)] present: {}", has_godot_class);
    println!("✅ #[class(...)] present: {}", has_class_attr);
    println!("✅ #[godot_api] present: {}", has_godot_api);
    println!("✅ Struct is public: {}", is_pub);

    if has_godot_class && has_class_attr && has_godot_api && is_pub {
        println!("🎉 Macro invocation chain looks correct!");
    } else {
        println!("⚠️ One or more macro sigils are missing or misaligned.");
    }
}
