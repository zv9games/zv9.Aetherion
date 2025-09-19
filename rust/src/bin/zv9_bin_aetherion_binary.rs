//c:/ZV9/zv9.aetherion/rust/src/zv9_bin_aetherion_binary.rs

#[macro_use]
extern crate aetherion_engine;
use aetherion_engine::util::logging::{init_logging, log_info};

use crossterm::event::{self, Event, KeyCode};
use std::thread;
use std::time::Duration;

mod zv9_bin_aetherion_binary_menu;
mod zv9_bin_aetherion_binary_func;

use zv9_bin_aetherion_binary_menu::{build_menu, print_menu};

fn main() {
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
// the end