//! ✅ Conductor Test Suite
//! Validates procedural orchestration, command queueing, tick flow, and signal dispatch.

#[cfg(test)]
mod conductor_tests {
    //use super::*;
    use crate::aetherion::core::conductor::{Conductor, ProcCommand};
    use crate::aetherion::pipeline::data::MapDataChunk;
    use crate::godot4::messaging::GodotSync;

    fn setup_conductor() -> Conductor {
        Conductor::new(GodotSync::init())
    }

    fn setup_chunk() -> MapDataChunk {
        MapDataChunk::default()
    }

    // 🧪 GenerateTerrain command
    #[test]
    fn generate_terrain_consumes_command() {
        let mut conductor = setup_conductor();
        let mut chunk = setup_chunk();

        conductor.enqueue(ProcCommand::GenerateTerrain);
        conductor.tick(0, &mut chunk);

        assert_eq!(conductor.queue_len(), 0, "GenerateTerrain should be consumed");
        println!("✅ generate_terrain_consumes_command");
    }

    // 🧪 OverlayStructure command
    #[test]
    fn overlay_structure_consumes_command() {
        let mut conductor = setup_conductor();
        let mut chunk = setup_chunk();

        conductor.enqueue(ProcCommand::OverlayStructure);
        conductor.tick(1, &mut chunk);

        assert_eq!(conductor.queue_len(), 0, "OverlayStructure should be consumed");
        println!("✅ overlay_structure_consumes_command");
    }

    // 🧪 EmitSignal command
    #[test]
    fn emit_signal_consumes_command() {
        let mut conductor = setup_conductor();
        let mut chunk = setup_chunk();

        conductor.enqueue(ProcCommand::EmitSignal("Hello, world!".to_string()));
        conductor.tick(2, &mut chunk);

        assert_eq!(conductor.queue_len(), 0, "EmitSignal should be consumed");
        println!("✅ emit_signal_consumes_command");
    }

    // 🧪 WaitTicks delay logic
    #[test]
    fn wait_ticks_delays_execution() {
        let mut conductor = setup_conductor();
        let mut chunk = setup_chunk();

        conductor.enqueue(ProcCommand::WaitTicks(3));
        conductor.tick(3, &mut chunk);
        conductor.tick(4, &mut chunk);
        conductor.tick(5, &mut chunk);

        assert!(conductor.has_pending(), "Should still be waiting");
        conductor.tick(6, &mut chunk);
        assert!(!conductor.has_pending(), "WaitTicks should be complete");
        println!("✅ wait_ticks_delays_execution");
    }

    // 🧪 ApplyModifier closure
    #[test]
    fn apply_modifier_executes_closure() {
        let mut conductor = setup_conductor();
        let mut chunk = setup_chunk();

        conductor.enqueue(ProcCommand::ApplyModifier(Box::new(|_chunk| {
            println!("🖌 Modifier applied.");
        })));

        conductor.tick(7, &mut chunk);
        assert_eq!(conductor.queue_len(), 0, "Modifier should be consumed");
        println!("✅ apply_modifier_executes_closure");
    }

    // 🧪 Queue length tracking
    #[test]
    fn queue_len_tracks_commands() {
        let mut conductor = setup_conductor();

        conductor.enqueue(ProcCommand::EmitSignal("A".to_string()));
        conductor.enqueue(ProcCommand::EmitSignal("B".to_string()));

        assert_eq!(conductor.queue_len(), 2, "Queue should contain 2 commands");
        println!("✅ queue_len_tracks_commands");
    }

    // 🧪 Pending status check
    #[test]
    fn has_pending_reports_correctly() {
        let mut conductor = setup_conductor();

        conductor.enqueue(ProcCommand::EmitSignal("Still pending".to_string()));
        assert!(conductor.has_pending(), "Should report pending commands");
        println!("✅ has_pending_reports_correctly");
    }

    // 🧪 Tick while waiting
    #[test]
    fn tick_while_waiting_resolves_after_delay() {
        let mut conductor = setup_conductor();
        let mut chunk = setup_chunk();

        conductor.enqueue(ProcCommand::WaitTicks(2));
        conductor.tick(8, &mut chunk);
        conductor.tick(9, &mut chunk);

        assert!(conductor.has_pending(), "Should still be waiting");
        conductor.tick(10, &mut chunk);
        assert!(!conductor.has_pending(), "Wait should be complete");
        println!("✅ tick_while_waiting_resolves_after_delay");
    }
}
