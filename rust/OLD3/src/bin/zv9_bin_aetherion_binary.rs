//c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary.rs



use aetherion_engine::util::logging::{init_logging, log_info};
use aetherion_engine::util::{build_menu, print_menu};
use aetherion_engine::log_component;

use crossterm::event::{self, Event, KeyCode};
use std::thread;
use std::time::Duration;

fn main() {
    use std::collections::HashSet;

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

    let menu = build_menu();
    print_menu(&menu);
    log_info("Console", "Awaiting menu selection...");

    let mut last_keys = HashSet::new();

    loop {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    if !last_keys.contains(&c) {
                        last_keys.insert(c);

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
            last_keys.clear(); // Reset key memory when no key is pressed
        }

        thread::sleep(Duration::from_millis(10));
    }

    log_info("Exit", "Engine shutdown complete.");
}



// the end