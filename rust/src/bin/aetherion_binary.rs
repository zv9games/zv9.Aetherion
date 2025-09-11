use aetherion_engine::util::logging::{init_logging, log_info};
use crossterm::event::{self, Event, KeyCode};
use std::{process::Command, thread, time::Duration};
use walkdir::WalkDir;

// Trailkeeper modules
use aetherion_engine::trailkeeper::{
    collector::Trailkeeper,
    config::check_config_change,
    scan::scan_git_diff,
    entry::LogEntry,
};

// 🧩 Menu item definition
struct MenuItem {
    key: char,
    label: &'static str,
    action: fn(),
}

// 📜 Log viewer
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

fn view_trailkeeper_logs() {
    use std::io::{self, Write};

    println!("\n📜 Trailkeeper Log Registry:\n");

    let logs = Trailkeeper::all();
    if logs.is_empty() {
        println!("(No logs recorded yet.)");
    } else {
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
    }

    println!("\n✅ Log inspection complete.\n");
}

// 📦 Module scanner
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

// 🧪 Runtime diagnostics
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

// 🌀 Tick simulation
fn simulate_tick_flow() {
    use aetherion_engine::{
        aetherion::core::conductor::{Conductor, ProcCommand},
        aetherion::pipeline::data::MapDataChunk,
        godot4::messaging::GodotSync,
    };

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

// 📋 Queue inspection
fn inspect_pending_queue() {
    use aetherion_engine::{
        aetherion::core::conductor::{Conductor, ProcCommand},
        godot4::messaging::GodotSync,
    };

    let mut conductor = Conductor::new(GodotSync::init());
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}

// 🔍 Trailkeeper scan
fn run_trailkeeper_scan() {
    println!("🔍 Running Trailkeeper scan...\n");
    scan_git_diff();
    check_config_change();

    for log in Trailkeeper::all() {
        println!("{:?}", log);
    }

    println!("\n✅ Trailkeeper scan complete.\n");
}

// 🧭 Menu builder
fn build_menu() -> Vec<MenuItem> {
    vec![
        MenuItem { key: '0', label: "Run: Cargo Test Suite", action: run_cargo_tests },
        MenuItem { key: '1', label: "Simulate: Tick Flow", action: simulate_tick_flow },
        MenuItem { key: '2', label: "Inspect: Pending Queue", action: inspect_pending_queue },
        MenuItem { key: '3', label: "Run: Trailkeeper Scan", action: run_trailkeeper_scan },
        MenuItem { key: '4', label: "View: Trailkeeper Logs", action: view_trailkeeper_logs },
        MenuItem { key: '9', label: "Exit", action: || {} },
    ]
}

// 🧭 Menu printer
fn print_menu(menu: &[MenuItem]) {
    println!("\n🧭 Aetherion Engine Dev Console\n");
    for item in menu {
        println!("[{}] {}", item.key, item.label);
    }
    println!("\nPress a number key to select an option...\n");
}

// 🚀 Main loop
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

    print_module_tree();

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
