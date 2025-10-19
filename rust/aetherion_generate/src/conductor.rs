// aetherion_generate/src/conductor.rs

use tokio::runtime::{Runtime, Handle};
use tracing::{info, error};
use std::collections::HashMap;
use std::sync::Arc;

// Local Crate Imports
use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
use crate::cellular_automata_generator::CellularAutomataGenerator;

// Shared Dependencies
use aetherion_math::Vec2i;
use aetherion_shared::chunk_data::ChunkData;


/// The central manager for the procedural generation pipeline.
/// It orchestrates tasks, manages the Tokio Runtime, and holds global state.
pub struct Conductor {
    runtime: Runtime,
    generators: HashMap<String, Arc<Box<dyn Generator>>>,
    // NOTE: This field is private, which is correct for internal state.
    // We add a public getter below to satisfy external crates (like aetherion_godot).
    active_generator_id: String,
    // TODO: Phase 2 - Add AtomicResource<AetherionState> for shared data access (as per manifest).
}
            
impl Conductor {
    pub fn new() -> Result<Self, std::io::Error> {
        // --- Runtime Setup (Phase 2 - Orchestration Core Complete) ---
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4) 
            .enable_all()
            .build()?;

        // --- Generator Initialization ---
        let mut map: HashMap<String, Arc<Box<dyn Generator>>> = HashMap::new();
        
        // 1. Perlin Generator (Minimal Viable Generator - MVG)
        let perlin: Box<dyn Generator> = Box::new(PerlinGenerator::new(64.0));
        // FIX: Capture the ID before moving the Box into the Arc
        let default_id = perlin.id().to_string(); 
        map.insert(default_id.clone(), Arc::new(perlin));

        // 2. Cellular Automata Generator
        let ca: Box<dyn Generator> = Box::new(CellularAutomataGenerator::new(184));
        map.insert(ca.id().to_string(), Arc::new(ca));
        
        // Set the MVG (Perlin) as the default active generator
        info!("Conductor initialized. Default generator set to: {}", default_id);

        Ok(Conductor {
            runtime,
            generators: map,
            active_generator_id: default_id,
        })
    }

    /// Public method to get a handle to the Runtime for task spawning.
    pub fn get_handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// **FIX for E0616:** Provides public read-access to the private field.
    /// This is necessary for aetherion_godot to implement its API surface.
    pub fn get_active_generator_id(&self) -> &str {
        &self.active_generator_id
    }
    
    /// Gracefully shuts down the Conductor and the underlying runtime.
    pub fn shutdown(self) {
        info!("Aetherion Conductor received shutdown signal. Stopping runtime.");
        self.runtime.shutdown_background(); 
        info!("Aetherion Conductor shutdown complete.");
    }

    // --- Generator Management & Core Pipeline ---

    /// Changes the algorithm used for subsequent generation tasks.
    pub fn set_active_generator(&mut self, id: &str) -> Result<(), String> {
        if self.generators.contains_key(id) {
            info!("Active generator set to: {}", id);
            self.active_generator_id = id.to_string();
            Ok(())
        } else {
            let err = format!("Generator ID '{}' not found. Available IDs: {:?}", id, self.generators.keys());
            error!("{}", err);
            Err(err)
        }
    }
    
    /// The primary function used by the runtime to generate a chunk.
    /// Executes the **Minimal Viable Generator (MVG) Logic**.
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        // Retrieve the currently active generator
        let generator_arc = self.generators
            .get(&self.active_generator_id)
            .expect("Active generator ID must be registered in Conductor.");
        
        // Dynamic dispatch to the concrete implementation (e.g., PerlinGenerator)
        info!("Conductor dispatching generation of chunk {:?} using '{}'", 
              chunk_coords, self.active_generator_id);
        
        // EXECUTE THE CORE GENERATION LOGIC
        let chunk_data = generator_arc.generate_chunk(chunk_coords);
        
        // TODO: Phase 2 - After MVG validation, insert the ChunkData into aetherion_cache::ChunkStore.
        chunk_data
    }
}