//! 📡 Echo API
//! External interface for invoking Aetherion Engine rituals.
//! Now refactored for asynchronous, non-blocking operation via threads.

use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};

use godot::prelude::{godot_api, godot_print, Base, GodotClass, Object, *};

use godot::builtin::GString;
use godot_core::classes::Node;

use crate::prelude::{EngineConfig, Tile, TileKind};
use crate::engine::lifecycle::{Lifecycle, Phase};
use crate::engine::runtime::Runtime;

// --- NEW: Communication Protocol ---
enum EngineCommand {
    Step,
    Generate,
    GetDebugInfo {
        respond_to: Sender<String>,
    },
    Shutdown,
}

// --- Godot ABI Wrapper ---
#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct EchoApi {
    runtime_thread: Option<JoinHandle<()>>,
    command_sender: Option<Sender<EngineCommand>>,
}

#[godot_api]
impl EchoApi {
    /// 🛑 No breath on mount — plugin must invoke `init_runtime()`
    fn init(_base: Base<Object>) -> Self {
        Self {
            runtime_thread: None,
            command_sender: None,
        }
    }
    
    // NEW: We need a way to gracefully shut down the thread when Godot closes.
    // The _notification function is perfect for this.
    fn _notification(&mut self, what: i32) {
		const NOTIFICATION_PREDELETE: i32 = 1; // Godot's internal enum value

		if what == NOTIFICATION_PREDELETE {
			godot_print!("EchoApi is being deleted. Shutting down Aetherion thread...");

			if let Some(sender) = &self.command_sender {
				let _ = sender.send(EngineCommand::Shutdown);
			}

			if let Some(handle) = self.runtime_thread.take() {
				match handle.join() {
					Ok(_) => godot_print!("✅ Aetherion thread shut down cleanly."),
					Err(e) => godot_print!("!! Aetherion thread panicked on shutdown: {:?}", e),
				}
			}
		}
	}


    /// 🧪 Manual runtime invocation — plugin must call this ONCE
    #[func]
    fn init_runtime(&mut self) {
        if self.runtime_thread.is_some() {
            godot_print!("⚠️ Aetherion runtime is already initialized.");
            return;
        }

        godot_print!("🚀 Initializing Aetherion runtime in a background thread...");

        let (tx, rx): (Sender<EngineCommand>, Receiver<EngineCommand>) = mpsc::channel();
        self.command_sender = Some(tx);

        let handle = thread::spawn(move || {
            godot_print!("🧵 Aetherion thread started.");

            let config = EngineConfig::default();
            let mut runtime = Runtime::new(config, true);

            for command in rx {
                match command {
                    EngineCommand::Step => {
                        runtime.step();
                    }
                    EngineCommand::Generate => {
                        runtime.cycle.advance(Phase::Generate);
                    }
                    EngineCommand::GetDebugInfo { respond_to } => {
                        let tick = runtime.cycle.tick;
                        let tile_count = runtime.cycle.registry.tiles.len();
                        let response = format!("Tick: {}\nTiles: {}", tick, tile_count);
                        respond_to.send(response).expect("Failed to send debug info response.");
                    }
                    EngineCommand::Shutdown => {
                        godot_print!("🧵 Shutdown command received. Exiting thread loop.");
                        break;
                    }
                }
            }
            godot_print!("🧵 Aetherion thread finished.");
        });

        self.runtime_thread = Some(handle);
    }

    /// ✅ Godot-facing method — checks if the communication channel is ready
    #[func]
    fn is_ready(&self) -> bool {
        self.command_sender.is_some()
    }

    /// 🔁 Advances Animate and Tick phases — sends a non-blocking command
    #[func]
    fn advance_tick(&self) {
        if let Some(sender) = &self.command_sender {
            sender.send(EngineCommand::Step).expect("Failed to send Step command.");
        }
    }

    /// 🧩 Optional: Generate phase invocation — sends a non-blocking command
    #[func]
    fn generate_tiles(&self) {
        if let Some(sender) = &self.command_sender {
            sender.send(EngineCommand::Generate).expect("Failed to send Generate command.");
        }
    }
    
    /// 📡 Godot-facing method — performs a blocking request-response to get data
    #[func]
    fn get_debug_output(&self) -> GString {
        if let Some(sender) = &self.command_sender {
            let (response_tx, response_rx) = mpsc::channel();
            let command = EngineCommand::GetDebugInfo { respond_to: response_tx };

            sender.send(command).expect("Failed to send GetDebugInfo command.");

            match response_rx.recv() {
                Ok(response) => response.into(),
                Err(_) => "Failed to receive response from engine thread.".into(),
            }
        } else {
            "Runtime not initialized.".into()
        }
    }
}