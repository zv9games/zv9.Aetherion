mod zv9_util_binary_func;
mod zv9_util_binary_func2;
mod zv9_util_binary_func3;
mod zv9_util_binary_menu;


use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use aetherion_core::zv9_util_logging::{init_logging, log_info};
use zv9_util_binary_menu::{build_menu, print_menu};
use zv9_util_binary_func::*;

use aetherion_core::log_component;

fn main() {
    // 🧠 Startup
    log_component!("AetherionBinary", "Interactive CLI for engine diagnostics and control");

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

    // 🧭 Menu setup
    let menu = build_menu();
    print_menu(&menu);
    log_info("Console", "Awaiting menu selection...");

    let mut last_keys = HashSet::new();

    // 🔁 Main loop
    loop {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    if last_keys.insert(c) {
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
        } else {
            last_keys.clear(); // 🧹 Reset key memory when idle
        }

        thread::sleep(Duration::from_millis(10));
    }

    // 🔒 Shutdown
    log_info("Exit", "Engine shutdown complete.");
}
