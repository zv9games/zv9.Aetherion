//! Implements a 2D Cellular Automata (CA) generator for pattern-based terrain,
//! including SSXL deterministic rules.

use crate::Generator; 
use aetherion_math::{
    Vec2i,
    generation_utils::fast_rand,
};
use aetherion_shared::{
    chunk_data::{ChunkData, CHUNK_SIZE},
    grid_bounds::GridBounds,
    tile_data::TileData,
    tile_type::TileType,
};
use tracing::{info, warn};

// --- CONSTANTS ---
const CA_ITERATIONS: u8 = 4;
const INITIAL_FILL_PERCENT: u8 = 45; // 45% of tiles start as 'Rock'

// --- RULESET DEFINITIONS (SSXL Integration) ---
pub const RULE_BASIC_CAVE: u8 = 0;   // CA Rule: Generates large, open cave systems.
pub const RULE_MAZE: u8 = 1;         // CA Rule: Generates thin, winding maze/pillar structures.
pub const RULE_SOLID: u8 = 2;        // SSXL Rule: Generates a completely solid chunk (non-CA).
pub const RULE_CHECKERBOARD: u8 = 3; // SSXL Rule: Generates a 1x1 checkerboard pattern (non-CA).


/// ⚙️ Implements a 2D Cellular Automata (CA) generator for pattern-based terrain.
#[allow(dead_code)]
pub struct CellularAutomataGenerator {
    /// The ID of the CA ruleset to use (e.g., 0 for Cave, 1 for Maze).
    ruleset: u8,
}

impl CellularAutomataGenerator {
    pub fn new(ruleset: u8) -> Self {
        CellularAutomataGenerator { ruleset }
    }
}

// --- CORE GENERATION LOGIC ---

/// Determines the next tile type based on the current type, live neighbors, and the active ruleset.
/// This function only applies to true Cellular Automata rules (CAVE, MAZE).
fn get_next_tile_type(current_type: TileType, live_neighbors: u8, ruleset: u8) -> TileType {
    // NOTE: We only handle Rock/Void transitions here.
    
    // Define Birth (B) and Survival (S) conditions based on the ruleset
    let (birth_min, birth_max, survive_min, survive_max) = match ruleset {
        RULE_MAZE => (3, 3, 1, 4), // B3/S1234: Cells need exactly 3 neighbors to be born, and survive if they have 1 to 4 neighbors.
        RULE_BASIC_CAVE | _ => (4, 5, 1, 7), // B45/S1234567: High birth rate (4 or 5) and high survival (1-7) for open caves.
    };
    
    match current_type {
        TileType::Rock => {
            // Survival Rule
            if live_neighbors >= survive_min && live_neighbors <= survive_max {
                TileType::Rock
            } else {
                TileType::Void
            }
        }
        TileType::Void => {
            // Birth Rule
            if live_neighbors >= birth_min && live_neighbors <= birth_max {
                TileType::Rock
            } else {
                TileType::Void
            }
        }
        // Preserve any other tile types (e.g., Water, Grass) that might be introduced later.
        _ => current_type,
    }
}

/// Applies one step of the Cellular Automata rule to the chunk grid.
fn apply_ca_step(chunk_data: &mut ChunkData, ruleset: u8) {
    let mut new_tiles: Vec<TileData> = chunk_data.tiles.iter().cloned().collect();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let index = (y * CHUNK_SIZE + x) as usize;
            let current_tile = &chunk_data.tiles[index];
            let live_neighbors = count_live_neighbors(chunk_data, x, y);

            let new_type = get_next_tile_type(
                current_tile.tile_type, 
                live_neighbors, 
                ruleset
            );
            
            // Retain existing noise value (no change during CA smoothing)
            new_tiles[index] = TileData::new(new_type, current_tile.noise_value);
        }
    }
    chunk_data.insert_tiles(new_tiles);
}

/// Counts the number of 'live' (TileType::Rock) neighbors for a given coordinate (Moore neighborhood).
fn count_live_neighbors(chunk_data: &ChunkData, cx: u32, cy: u32) -> u8 {
    let mut count = 0;
    
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;

            // Check if neighbor is within chunk bounds
            if nx >= 0 && nx < CHUNK_SIZE as i32 && ny >= 0 && ny < CHUNK_SIZE as i32 {
                let index = (ny as u32 * CHUNK_SIZE + nx as u32) as usize;
                
                if chunk_data.tiles[index].tile_type == TileType::Rock {
                    count += 1;
                }
            }
        }
    }
    count
}

// --- TRAIT IMPLEMENTATION ---

impl Generator for CellularAutomataGenerator {
    fn id(&self) -> &str {
        match self.ruleset {
            RULE_MAZE => "cellular_automata_maze",
            RULE_SOLID => "cellular_automata_solid",
            RULE_CHECKERBOARD => "cellular_automata_checkerboard",
            RULE_BASIC_CAVE | _ => "cellular_automata_basic",
        }
    }

    fn generate_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        info!("CA Generator: Starting chunk generation at {:?} with ruleset {}.", chunk_coords, self.ruleset);

        // --- 1. CHUNK METADATA INITIALIZATION ---
        let chunk_tile_size = CHUNK_SIZE as i32;

        let world_start_x = chunk_coords.x * chunk_tile_size;
        let world_start_y = chunk_coords.y * chunk_tile_size;

        let chunk_id = (chunk_coords.x as u64) | ((chunk_coords.y as u64) << 32);

        let bounds = GridBounds::new(
            world_start_x as i64,
            world_start_y as i64,
            (world_start_x + chunk_tile_size) as i64,
            (world_start_y + chunk_tile_size) as i64,
        );

        let dimension_name = self.id().to_string(); // Use the ID as the dimension name

        let mut chunk_data = ChunkData::new(chunk_id, bounds, dimension_name);
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        
        // --- 2. SSXL Deterministic Rules (Bypass CA Iteration) ---
        match self.ruleset {
            RULE_SOLID => {
                info!("CA Generator: Using RULE_SOLID (SSXL deterministic fill).");
                for _ in 0..(CHUNK_SIZE * CHUNK_SIZE) {
                    // Fill all with Rock, using 1.0 noise for easy visual confirmation
                    tiles.push(TileData::new(TileType::Rock, 1.0)); 
                }
                chunk_data.insert_tiles(tiles);
                warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
                return chunk_data;
            },
            RULE_CHECKERBOARD => {
                info!("CA Generator: Using RULE_CHECKERBOARD (SSXL deterministic pattern).");
                for cy in 0..CHUNK_SIZE {
                    for cx in 0..CHUNK_SIZE {
                        // Checkerboard pattern depends on parity of coordinates
                        let is_rock = (cx + cy) % 2 == 0;
                        let tile_type = if is_rock { TileType::Rock } else { TileType::Void };
                        // Use 0.5 noise value for subtle patterning
                        tiles.push(TileData::new(tile_type, 0.5)); 
                    }
                }
                chunk_data.insert_tiles(tiles);
                warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
                return chunk_data;
            },
            _ => { /* Fallthrough to standard CA logic */ }
        }

        // --- 3. Standard CA Logic (Initial Random Fill) ---
        for _ in 0..(CHUNK_SIZE * CHUNK_SIZE) {
            // fast_rand(N) returns 0 if a random number 0..=99 is < N
            let is_rock = fast_rand(INITIAL_FILL_PERCENT) == 0; 

            let tile_type = if is_rock {
                TileType::Rock
            } else {
                TileType::Void
            };
            tiles.push(TileData::new(tile_type, 0.0));
        }
        chunk_data.insert_tiles(tiles);

        // --- 4. APPLY CA ITERATIONS ---
        for i in 0..CA_ITERATIONS {
            // Pass the active ruleset ID
            apply_ca_step(&mut chunk_data, self.ruleset); 
            info!("CA Generator: Iteration {} complete.", i + 1);
        }

        warn!("CA Generator: Finished chunk at {:?}. Result is ready.", chunk_coords);
        chunk_data
    }
}