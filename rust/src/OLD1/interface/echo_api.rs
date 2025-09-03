// src/interface/echo_api.rs

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use once_cell::sync::{OnceCell, Lazy};
use godot::prelude::*;
use godot::classes::Node;

use crate::model::{self, DebugInfo, ThreadSafeDebugInfo};

// We use `Lazy` for the thread-safe `Sender`.
static SENDER: Lazy<Sender<model::Message>> = Lazy::new(|| {
    let (tx, rx) = channel();
    // We store the receiver in a OnceCell so the main thread can get it.
    RECEIVER.set(rx).expect("Failed to set receiver");
    tx
});

// The Receiver is stored in a OnceCell that the main thread gets access to.
// This is the key: we do not try to make the Receiver Sync.
static RECEIVER: OnceCell<Receiver<model::Message>> = OnceCell::new();

#[derive(GodotClass)]
#[class(base=Node)]
pub struct EchoApi {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl EchoApi {
    // The `init` constructor is a standard Rust method, not a Godot API function.
    #[func]
    fn init(base: Base<Node>) -> Self {
        // We initialize the channel lazily.
        let _ = SENDER.get();
        Self {
            base,
        }
    }

    // This function spawns a thread and gives it a clone of the sender.
    // Call this from Godot to start a new background task.
    #[func]
    pub fn start_background_task(&mut self) {
        // Get a reference to the Sender and clone it for the new thread.
        let sender_clone = SENDER.get().expect("Channel sender not initialized").clone();

        std::thread::spawn(move || {
            // In the background thread, we can now send messages.
            let info = ThreadSafeDebugInfo {
                message: "Hello from the background thread!".to_string(),
            };
            sender_clone.send(model::Message::DebugInfo(info)).unwrap();
            
            // For example, send another message after a delay.
            std::thread::sleep(std::time::Duration::from_secs(2));
            sender_clone.send(model::Message::MapLoadComplete).unwrap();
        });
    }
    
    // The #[signal] macro now takes the signal name and the arguments.
    #[signal]
    fn debug_info_received(info: Gd<godot::classes::RefCounted>);
    
    #[signal]
    fn map_load_complete();

    #[func]
    pub fn _process(&mut self, _delta: f64) {
        if let Some(rx) = RECEIVER.get() {
            // We use a loop to process all available messages without blocking.
            loop {
                match rx.try_recv() {
                    Ok(msg) => {
                        match msg {
                            model::Message::DebugInfo(info) => {
                                // We convert the thread-safe data into a Godot object on the main thread.
                                let gd_info = DebugInfo::from_thread_safe(info);
                                self.base_mut().emit_signal("debug_info_received".into(), &[gd_info.to_variant()]);
                            }
                            model::Message::MapLoadComplete => {
                                self.base_mut().emit_signal("map_load_complete".into(), &[]);
                            }
                        }
                    }
                    // We stop when the channel is empty.
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        godot_warn!("Channel disconnected!");
                        // Since the `RECEIVER` is a static `OnceCell`, we can't clean it up here.
                        // However, the error is non-fatal and the loop will simply stop processing.
                        break;
                    }
                }
            }
        }
    }
}
