// tests/core/conductor_tests.rs

//! ✅ Conductor Test Suite
//! Validates procedural orchestration, command queueing, tick flow, and signal dispatch.

use Aetherion_Engine::aetherion::core::conductor::{Conductor, ProcCommand};
use Aetherion_Engine::aetherion::pipeline::data::MapDataChunk;
use Aetherion_Engine::godot4::messaging::{GodotSync, EngineMessage};

pub fn run_all() {
    println!("🔧 Running Conductor test suite...\n");

    test_generate_terrain();
    test_overlay_structure();
    test_emit_signal();
    test_wait_ticks();
    test_apply_modifier();
    test_queue_len();
    test_has_pending();
    test_tick_while_waiting();

    println!("\n✅ All Conductor tests passed.\n");
}

fn test_generate_terrain() {
    let mut conductor = Conductor::new(GodotSync::new());
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::GenerateTerrain);
    conductor.tick(0, &mut chunk);

    assert_eq!(conductor.queue_len(), 0, "GenerateTerrain should be consumed");
    println!("✅ test_generate_terrain");
}

fn test_overlay_structure() {
    let mut conductor = Conductor::new(GodotSync::new());
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::OverlayStructure);
    conductor.tick(1, &mut chunk);

    assert_eq!(conductor.queue_len(), 0, "OverlayStructure should be consumed");
    println!("✅ test_overlay_structure");
}

fn test_emit_signal() {
    let mut conductor = Conductor::new(GodotSync::new());
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::EmitSignal("Hello, world!".to_string()));
    conductor.tick(2, &mut chunk);

    assert_eq!(conductor.queue_len(), 0, "EmitSignal should be consumed");
    println!("✅ test_emit_signal");
}

fn test_wait_ticks() {
    let mut conductor = Conductor::new(GodotSync::new());
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::WaitTicks(3));
    conductor.tick(3, &mut chunk);
    conductor.tick(4, &mut chunk);
    conductor.tick(5, &mut chunk);

    assert!(conductor.has_pending(), "Should still be waiting");
    conductor.tick(6, &mut chunk);
    assert!(!conductor.has_pending(), "WaitTicks should be complete");
    println!("✅ test_wait_ticks");
}

fn test_apply_modifier() {
    let mut conductor = Conductor::new(GodotSync::new());
    let mut chunk = MapDataChunk::default();

    let mut modified = false;
    conductor.enqueue(ProcCommand::ApplyModifier(Box::new(|_chunk| {
        // Simulate mutation
        println!("🖌 Modifier applied.");
    })));

    conductor.tick(7, &mut chunk);
    assert_eq!(conductor.queue_len(), 0, "Modifier should be consumed");
    println!("✅ test_apply_modifier");
}

fn test_queue_len() {
    let mut conductor = Conductor::new(GodotSync::new());

    conductor.enqueue(ProcCommand::EmitSignal("A".to_string()));
    conductor.enqueue(ProcCommand::EmitSignal("B".to_string()));

    assert_eq!(conductor.queue_len(), 2, "Queue should contain 2 commands");
    println!("✅ test_queue_len");
}

fn test_has_pending() {
    let mut conductor = Conductor::new(GodotSync::new());

    conductor.enqueue(ProcCommand::EmitSignal("Still pending".to_string()));
    assert!(conductor.has_pending(), "Should report pending commands");
    println!("✅ test_has_pending");
}

fn test_tick_while_waiting() {
    let mut conductor = Conductor::new(GodotSync::new());
    let mut chunk = MapDataChunk::default();

    conductor.enqueue(ProcCommand::WaitTicks(2));
    conductor.tick(8, &mut chunk);
    conductor.tick(9, &mut chunk);

    assert!(conductor.has_pending(), "Should still be waiting");
    conductor.tick(10, &mut chunk);
    assert!(!conductor.has_pending(), "Wait should be complete");
    println!("✅ test_tick_while_waiting");
}
