// aetherion_generate/src/cellular_automata_generator.rs

// --- ADDED IMPORTS ---
use crate::Generator;
use aetherion_math::Vec2i;
// Added CHUNK_SIZE and imported GridBounds
use aetherion_shared::chunk_data::{ChunkData, CHUNK_SIZE};
use aetherion_shared::grid_bounds::GridBounds;

pub struct CellularAutomataGenerator {
    // CA specific configuration (e.g., initial density, rule set)
    ruleset: u8,
}

impl CellularAutomataGenerator {
    pub fn new(ruleset: u8) -> Self {
        CellularAutomataGenerator { ruleset }
    }
}

impl Generator for CellularAutomataGenerator {
    fn id(&self) -> &str {
        "cellular_automata_basic"
    }

    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        println!("  -> Generating CA Chunk at {:?}", chunk_coords);

        // --- NEW CODE BLOCK: CONSTRUCTOR ARGUMENTS ---
        let chunk_tile_size = CHUNK_SIZE as i32;

        // Calculate world-space boundaries based on chunk coordinates
        let world_start_x = chunk_coords.x * chunk_tile_size;
        let world_start_y = chunk_coords.y * chunk_tile_size;

        // 1. Chunk ID (u64 hash)
        let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);

        // 2. GridBounds (min_x, min_y, max_x, max_y)
        let bounds = GridBounds::new(
            world_start_x as i64, 
            world_start_y as i64, 
            (world_start_x + chunk_tile_size) as i64, 
            (world_start_y + chunk_tile_size) as i64
        ); 
        
        // 3. Dimension Name
        let dimension_name = "CA_Automata".to_string(); 
        
        // Final call to the constructor
        ChunkData::new(chunk_id, bounds, dimension_name) 
    }
}