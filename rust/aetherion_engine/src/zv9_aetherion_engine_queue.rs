use crate::core::conductor::{Conductor, ProcCommand};
use crate::zv9_godot_interface_messaging_sync::GodotSync;

/// 📋 Inspects the procedural command queue using GodotSync
pub fn inspect_pending_queue() {
    let mut conductor = Conductor::new(GodotSync::init());
    conductor.enqueue(ProcCommand::EmitSignal("Pending check".into()));

    println!("📋 Queue length: {}", conductor.queue_len());
    println!("⏳ Has pending: {}", conductor.has_pending());
    println!("✅ Queue inspection complete.\n");
}
