use Aetherion_Engine::util::logging::{init_logging, log_info};
use crossterm::event::{self, Event, KeyCode};
use std::process::Command;
use std::time::Duration;
use walkdir::WalkDir;

// 🧩 Menu item definition
struct MenuItem {
    key: char,
    label: &'static str,
    action: fn(),
}

// 📦 Module scanner
fn print_module_tree() {
    println!("\n📦 Scanning for Rust modules in /src...\n");

    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path().display();
        println!("├── {}", path);
    }

    println!("\n✅ Module scan complete.\n");
}

// 🧪 Runtime diagnostics

fn run_cargo_tests() {
    println!("🚀 Running full cargo test suite...\n");
    let status = Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--nocapture")
        .status()
        .expect("Failed to run cargo test");

    if status.success() {
        println!("✅ All tests passed.");
    } else {
        println!("❌ Some tests failed.");
    }
}

fn simulate_tick_flow() {
    use Aetherion_Engine::aetherion::core::conductor::{Conductor, ProcCommand};
    use Aetherion_Engine::aetherion::pipeline::data::MapDataChunk;
    use Aetherion_Engine::godot4::messaging::GodotSync;

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
    use Aetherion_Engine::aetherion::core::conductor::{Conductor, ProcCommand};
    use Aetherion_Engine::godot4::messaging::GodotSync;

    let mut conductor = Conductor::new(GodotSync::init());
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}

// 🧭 Menu builder
fn build_menu() -> Vec<MenuItem> {
    vec![
        MenuItem { key: '0', label: "Run: Cargo Test Suite", action: run_cargo_tests },
        MenuItem { key: '1', label: "Simulate: Tick Flow", action: simulate_tick_flow },
        MenuItem { key: '2', label: "Inspect: Pending Queue", action: inspect_pending_queue },
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

        std::thread::sleep(Duration::from_millis(10));
    }

    log_info("Exit", "Engine shutdown complete.");
}
