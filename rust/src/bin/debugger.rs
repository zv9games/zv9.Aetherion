use eframe::{egui, App, CreationContext, NativeOptions};
use std::time::{Duration, Instant};
use chrono::Local;

/// 🧿 DebuggerApp — pulses every 0.5s with current time
pub struct DebuggerApp {
    last_time: String,
    last_update: Instant,
}

impl Default for DebuggerApp {
    fn default() -> Self {
        Self {
            last_time: current_time_string(),
            last_update: Instant::now(),
        }
    }
}

impl App for DebuggerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_millis(500) {
            self.last_time = current_time_string();
            self.last_update = now;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🧿 AETHERION DEBUG WINDOW");
            ui.label(format!("Current Time: {}", self.last_time));
        });

        ctx.request_repaint(); // Ensures smooth ticking
    }
}

/// 🕰️ Formats current time as string
fn current_time_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 🪶 Main entry — launches native observatory window
fn main() {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("🧿 Aetherion Debugger")
            .with_inner_size([320.0, 120.0])
            .with_always_on_top()
            .with_decorations(true),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "🧿 Aetherion Debugger",
        native_options,
        Box::new(|_cc: &CreationContext| Box::<DebuggerApp>::default()),
    );
}
