//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_structure_placements.rs

use crate::zv9_prelude::*;

/// 🏗 Dummy structure placement function for testing and integration.
pub fn place_structure_stub(grid: &mut MapGrid, pos: Position) {
    // Define a 3x3 region starting from the given position
    let bounds = GridBounds::new(pos.to_vec2i().into(), Vector2i::new(3, 3).into());
    let tile_type = TileType::Chunk;

    // Place tiles within bounds
    for p in bounds.iter() {
        let position = Position { x: p.x, y: p.y };
        grid.set(position, tile_type);
    }

    // Log the placement event
    Trailkeeper::record(LogEntry {
        event_type: EventType::StructurePlacement,
        timestamp: chrono::Utc::now(),
        actor: "stub".into(),
        description: format!("Placed dummy structure at {:?}", pos),
        affected_components: vec![format!("{:?}", bounds)],
        status: LogStatus::Success,
    });
}

#[cfg(test)]
mod stress_tests {
    use super::*;
    use crate::zv9_prelude::*;

    #[test]
    fn stress_structure_placement_centered() {
        let mut grid = MapGrid::new(64, 64);
        let pos = Position { x: 30, y: 30 };
        place_structure_stub(&mut grid, pos);

        let bounds = GridBounds::new(pos.to_vec2i().into(), Vector2i::new(3, 3).into());
        for p in bounds.iter() {
            let placed = grid.get(Position { x: p.x, y: p.y });
            assert_eq!(placed, Some(TileType::Chunk));
        }
    }

    #[test]
    fn stress_structure_placement_near_edge() {
        let mut grid = MapGrid::new(8, 8);
        let pos = Position { x: 6, y: 6 };
        place_structure_stub(&mut grid, pos);

        let bounds = GridBounds::new(pos.to_vec2i().into(), Vector2i::new(3, 3).into());
        for p in bounds.iter() {
            let placed = grid.get(Position { x: p.x, y: p.y });
            // Should not panic even if out of bounds
            let _ = placed; // Optional: assert if within bounds
        }
    }

    #[test]
    fn stress_multiple_stub_placements() {
        let mut grid = MapGrid::new(128, 128);
        for i in 0..10 {
            let pos = Position { x: i * 10, y: i * 10 };
            place_structure_stub(&mut grid, pos);
        }

        let total_chunks = grid.count_type(TileType::Chunk);
        assert!(total_chunks >= 90); // 10 placements × ~9 tiles each
    }
}



// the end