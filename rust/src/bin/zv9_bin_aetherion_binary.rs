//c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary.rs




use aetherion_engine::core::runtime::start as start_runtime;
use aetherion_engine::util::logging::{init_logging, log_info};
use aetherion_engine::trailkeeper::{
    collector::Trailkeeper,
    config::check_config_change,
    scan::scan_git_diff,
    entry::LogEntry,
};
use aetherion_engine::pipeline_builder::bitmask::convert_world_png_to_chunk;
use aetherion_engine::{Conductor, ProcCommand, MapDataChunk, GodotSync, TileInfo};

use crossterm::event::{self, Event, KeyCode};
use std::{process::Command, thread, time::Duration};
use walkdir::WalkDir;

/// 🧩 Menu item definition
struct MenuItem {
    key: char,
    label: &'static str,
    action: fn(),
}

//
// ─── Menu Actions ─────────────────────────────────────────────────────────────
//

fn run_cargo_tests() {
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

fn simulate_tick_flow() {
    println!("🌀 Simulating tick flow with procedural commands...\n");

    let mut conductor = Conductor::new(GodotSync::init());
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::GenerateTerrain);
    conductor.enqueue(ProcCommand::EmitSignal("Hello from tick".into()));
    conductor.enqueue(ProcCommand::WaitTicks(2));

    for tick in 0..4 {
        println!("⏱ Tick {}", tick);
        conductor.tick(tick, &mut chunk);
    }

    println!("\n✅ Tick simulation complete.\n");
}

fn inspect_pending_queue() {
    let mut conductor = Conductor::new(GodotSync::init());
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}

fn run_trailkeeper_scan() {
    println!("🔍 Running Trailkeeper scan...\n");
    scan_git_diff();
    check_config_change();

    for log in Trailkeeper::all() {
        println!("{:?}", log);
    }

    println!("\n✅ Trailkeeper scan complete.\n");
}

fn view_trailkeeper_logs() {
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

fn print_log_entry(index: usize, log: &LogEntry) {
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

fn print_module_tree() {
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

fn run_max_grid_benchmark() {
    use std::time::{Instant, Duration};

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
        // Simulate tile placement
        chunk.place_tile(x, y, TileInfo::default());
        tiles_placed += 1;

        x += 1;
        if x >= grid_width {
            x = 0;
            y += 1;
        }

        // Log once per second
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


//
// ─── Menu Setup ───────────────────────────────────────────────────────────────
//

fn build_menu() -> Vec<MenuItem> {
    vec![
        MenuItem { key: '0', label: "Run: Cargo Test Suite", action: run_cargo_tests },
        MenuItem { key: '1', label: "Simulate: Tick Flow", action: simulate_tick_flow },
        MenuItem { key: '2', label: "Inspect: Pending Queue", action: inspect_pending_queue },
        MenuItem { key: '3', label: "Run: Trailkeeper Scan", action: run_trailkeeper_scan },
        MenuItem { key: '4', label: "View: Trailkeeper Logs", action: view_trailkeeper_logs },
        MenuItem { key: '5', label: "Start: Aetherion Runtime", action: start_runtime },
        MenuItem { key: '6', label: "Benchmark: Max Grid Placement", action: run_max_grid_benchmark },
		MenuItem { key: '7', label: "Perform: Bitmask PNG Conversion", action: run_bitmask_conversion },
        MenuItem { key: '9', label: "Exit", action: || {} },
    ]
}


fn print_menu(menu: &[MenuItem]) {
    println!("\n🧭 Aetherion Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nPress a number key to select an option...\n");
}

//
// ─── Main Loop ────────────────────────────────────────────────────────────────
//

fn main() {
    init_logging();
    log_info("Startup", "Engine boot sequence initiated.");

    println!(
        r#"
         (__)
         (oo)
  /-------\/
 / |     ||
*  ||----||
   ~~    ~~
    Aetherion Bull Initialized
"#
    );

    //print_module_tree();

    let menu = build_menu();
    print_menu(&menu);
    log_info("Console", "Awaiting menu selection...");

    loop {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    if let Some(item) = menu.iter().find(|m| m.key == c) {
                        log_info("Menu", &format!("Selected: {}", item.label));
                        (item.action)();
                        if c == '9' {
                            break;
                        }
                        print_menu(&menu);
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    log_info("Exit", "Engine shutdown complete.");
}

fn run_bitmask_conversion() {
    use std::time::Instant;

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