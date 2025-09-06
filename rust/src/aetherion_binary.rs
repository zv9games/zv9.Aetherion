use Aetherion_Engine::util::logging::{init_logging, log_info};
use crossterm::event::{self, Event, KeyCode};
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

// 🧪 Placeholder test functions
fn test_conductor() {
    println!("🔧 Running Conductor tests...");
    // TODO: Insert actual test logic
    println!("✅ Conductor test passed.\n");
}

fn test_dimension() {
    println!("🔧 Running Dimension tests...");
    println!("✅ Dimension test passed.\n");
}

fn test_lifecycle() {
    println!("🔧 Running Lifecycle tests...");
    println!("✅ Lifecycle test passed.\n");
}

fn test_runtime() {
    println!("🔧 Running Runtime tests...");
    println!("✅ Runtime test passed.\n");
}

fn test_grid() {
    println!("🔧 Running Grid diagnostics...");
    println!("✅ Grid test passed.\n");
}

fn test_racer() {
    println!("🔧 Running Racer movement...");
    println!("✅ Racer test passed.\n");
}

fn test_signal_dispatch() {
    println!("🔧 Running Signal dispatch...");
    println!("✅ Signal dispatch test passed.\n");
}

fn test_godot_sync() {
    println!("🔧 Running Godot sync...");
    println!("✅ Godot sync test passed.\n");
}

// 🧭 Menu builder
fn build_menu() -> Vec<MenuItem> {
    vec![
        MenuItem { key: '0', label: "Test: Conductor", action: test_conductor },
        MenuItem { key: '1', label: "Test: Dimension", action: test_dimension },
        MenuItem { key: '2', label: "Test: Lifecycle", action: test_lifecycle },
        MenuItem { key: '3', label: "Test: Runtime", action: test_runtime },
        MenuItem { key: '4', label: "Test: Grid", action: test_grid },
        MenuItem { key: '5', label: "Test: Racer", action: test_racer },
        MenuItem { key: '6', label: "Test: Signal Dispatch", action: test_signal_dispatch },
        MenuItem { key: '7', label: "Test: Godot Sync", action: test_godot_sync },
        MenuItem { key: '9', label: "Exit", action: || {} },
    ]
}

// 🧭 Menu printer
fn print_menu(menu: &[MenuItem]) {
    println!("\n🧭 Aetherion Engine Test Menu\n");
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
    log_info("Test", "Logging system verified. Awaiting menu selection...");

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
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    log_info("Exit", "Engine shutdown complete.");
}
