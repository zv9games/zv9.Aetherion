//c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary_func.rs
use aetherion_engine::trailkeeper::{
    collector::Trailkeeper,
    config::check_config_change,
    scan::scan_git_diff,
    entry::{EventType, LogEntry},
};
use aetherion_engine::pipeline_builder::bitmask::convert_world_png_to_chunk;
use aetherion_engine::{Conductor, ProcCommand, MapDataChunk, GodotSync, TileInfo};
use walkdir::WalkDir;
#[allow(unused_imports)]
use std::{collections::HashMap, fs, process::Command};

use std::{time::{Instant, Duration}};

pub fn run_cargo_tests() {
    println!("🚀 Running full cargo test suite...\n");
    let status = Command::new("cargo")
        .args(&["test", "--", "--nocapture"])
        .status()
        .expect("Failed to run cargo test");

    match status.success() {
        true => println!("✅ All tests passed."),
        false => println!("❌ Some tests failed."),
    }
}

pub fn inspect_pending_queue() {
    let mut conductor = Conductor::new(GodotSync::init());
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}

pub fn run_trailkeeper_scan() {
    println!("🔍 Running Trailkeeper scan...\n");
    scan_git_diff();
    check_config_change();

    for log in Trailkeeper::all() {
        println!("{:?}", log);
    }

    println!("\n✅ Trailkeeper scan complete.\n");
}

pub fn view_trailkeeper_logs() {
    use std::io::{self, Write};

    println!("\n📜 Trailkeeper Log Registry:\n");

    let logs = Trailkeeper::all();
    if logs.is_empty() {
        println!("(No logs recorded yet.)");
        return;
    }

    let stdin = io::stdin();
    let mut buffer = String::new();

    for (i, log) in logs.iter().enumerate() {
        print_log_entry(i + 1, log);
        print!("Press Enter to continue, or type 9 to quit: ");
        io::stdout().flush().unwrap();
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();

        if buffer.trim() == "9" {
            println!("\n🚪 Exiting log viewer...\n");
            break;
        }
    }

    println!("\n✅ Log inspection complete.\n");
}

pub fn print_log_entry(index: usize, log: &LogEntry) {
    println!("──────────────────────────────────────────────");
    println!("📄 Entry #{}", index);
    println!("🕒 Timestamp: {}", log.timestamp.to_rfc3339());
    println!("🧠 Event Type: {:?}", log.event_type);
    println!("👤 Actor: {}", log.actor);
    println!("📝 Description: {}", log.description);
    println!("📦 Components: {:?}", log.affected_components);
    println!("⚠️ Status: {:?}", log.status);
    println!("──────────────────────────────────────────────");
}

pub fn print_module_tree() {
    println!("\n📦 Scanning for Rust modules in /src...\n");

    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        println!("├── {}", entry.path().display());
    }

    println!("\n✅ Module scan complete.\n");
}

pub fn print_godot_api_surface() {
    use walkdir::WalkDir;
    use std::{collections::HashMap, fs};
    use regex::Regex;

    println!("🧪 API scan triggered");
    println!("📡 Recursively scanning for GDScript-callable API...\n");

    let class_regex = Regex::new(r"#\[\s*(derive\s*\(\s*GodotClass\s*\)|class\s*\(.*?\))\s*\]").unwrap();
    let method_regex = Regex::new(r"#\[\s*(func|method)\s*\]").unwrap();
    let fn_signature = Regex::new(r"^\s*(pub\s+)?fn\s+(\w+)\s*\(([^)]*)\)\s*(->\s*.+)?").unwrap();
    let struct_regex = Regex::new(r"^\s*(pub\s+)?struct\s+(\w+)").unwrap();
    let impl_regex = Regex::new(r"^\s*impl\s+(\w+)").unwrap();

    let ignored_impls = ["crate", "std", "Default", "From", "fmt", "WithSignals"];

    let mut current_class = String::new();
    let mut godot_api: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut file_count = 0;

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let path_str = e.path().display().to_string();
            e.path().extension().map_or(false, |ext| ext == "rs")
                && !path_str.contains("OLD1")
                && !path_str.contains("OLD2")
        })
    {
        file_count += 1;
        let path = entry.path();
        let file_path = path.display().to_string();

        if let Ok(content) = fs::read_to_string(path) {
            let mut lines = content.lines().peekable();
            let mut _line_number = 0;

            while let Some(line) = lines.next() {
                _line_number += 1;
                let line = line.trim();

                if line.contains("error[E0277]") {
                    continue;
                }

                // Match struct after class annotation
                if class_regex.is_match(line) {
                    for _ in 0..20 {
                        if let Some(next_line) = lines.peek() {
                            let next_line = next_line.trim();
                            if let Some(caps) = struct_regex.captures(next_line) {
                                if let Some(name) = caps.get(2) {
                                    current_class = name.as_str().to_string();
                                    break;
                                }
                            }
                            lines.next();
                        }
                    }
                }

                // Match impl block and filter noise
                if let Some(caps) = impl_regex.captures(line) {
                    let candidate = caps[1].to_string();
                    if !ignored_impls.contains(&candidate.as_str()) {
                        current_class = candidate;
                    }
                }

                // Match GDScript-callable method
                if method_regex.is_match(line) {
                    let mut signature_lines = Vec::new();
                    let mut lookahead = lines.clone();
                    for _ in 0..5 {
                        if let Some(next_line) = lookahead.next() {
                            let trimmed = next_line.trim();
                            signature_lines.push(trimmed.to_string());
                            if trimmed.contains('{') || trimmed.contains(';') {
                                break;
                            }
                        }
                    }
                    let joined = signature_lines.join(" ");
                    if let Some(caps) = fn_signature.captures(&joined) {
                        let name = &caps[2];
                        let params = &caps[3];
                        let return_type = caps.get(4).map_or("", |m| m.as_str());
                        let signature = format!("fn {}({}) {}", name, params, return_type);
                        if !current_class.is_empty() {
                            let key = (current_class.clone(), file_path.clone());
                            godot_api.entry(key).or_default().push(signature);
                        }
                    }
                }
            }
        }
    }

    println!("\n📘 GDScript-Callable Methods");

    if godot_api.is_empty() {
        println!("(No GDScript-callable methods detected. Scanned {} files.)", file_count);
    } else {
        for ((class, file), methods) in godot_api {
            println!("\n🔹 Node: {}  📁 {}", class, file);
            for method in methods {
                println!("   └── {}", method);
            }
        }
    }

    println!("\n✅ GDScript-callable methods printed.\n");

    log_event!(
        EventType::System,
        "AetherionBinary",
        "Scanned GDScript-callable API surface"
    );
}









pub fn run_max_grid_benchmark() {
    println!("🧪 Starting max grid benchmark (30s)...");

    let mut chunk = MapDataChunk::default();
    let start = Instant::now();
    let time_limit = Duration::from_secs(30);
    let mut tiles_placed = 0;
    let mut last_logged = Instant::now();

    let grid_width = 10_000; // virtual width, not allocated
    let mut x = 0;
    let mut y = 0;

    while Instant::now() - start < time_limit {
        chunk.place_tile(x, y, TileInfo::default());
        tiles_placed += 1;

        x += 1;
        if x >= grid_width {
            x = 0;
            y += 1;
        }

        if Instant::now() - last_logged >= Duration::from_secs(1) {
            println!("⏱ {}s elapsed — {} tiles placed", 
                (Instant::now() - start).as_secs(),
                tiles_placed
            );
            last_logged = Instant::now();
        }
    }

    println!("\n✅ Benchmark complete.");
    println!("🧱 Total tiles placed: {}", tiles_placed);
    println!("📐 Final grid size: {} x {}", grid_width, y + 1);
    println!("⚡ Throughput: ~{} tiles/sec", tiles_placed / 30);
}

pub fn run_bitmask_conversion() {
    println!("🧪 Starting bitmask conversion from world.png...");

    let path = "C:/ZV9/zv9.aetherion/.assets/world.png";
    let scale = 3;
    let start = Instant::now();

    let chunk = convert_world_png_to_chunk(path, scale);
    let elapsed = start.elapsed();

    println!("✅ Conversion complete.");
    println!("🧱 Tiles placed: {}", chunk.len());
    println!("📐 Final grid size: ~{} x {}", 
        (chunk.len() as f64).sqrt().round() as u32, 
        (chunk.len() as f64).sqrt().round() as u32
    );
    println!("⏱ Time taken: {:.2?}", elapsed);
}
// the end
