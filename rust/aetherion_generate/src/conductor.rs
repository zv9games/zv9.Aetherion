//! The core manager for the Aetherion Engine, responsible for coordinating
//! generation, concurrency, and caching via the Tokio asynchronous runtime.

use tokio::runtime::{Runtime, Handle};
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::io;

// NEW: Cache and Math imports for Phase 6
use aetherion_cache::ChunkCache;
use aetherion_math::Vec2i;
use aetherion_math::prelude::ChunkKey; // FIX: Resolved E0432 for ChunkKey based on compiler hint
use glam::IVec3; // Used to construct ChunkKey from Vec2i

// --- INTERNAL CRATE DEPENDENCIES ---
use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
use crate::cellular_automata_generator::{
    CellularAutomataGenerator, 
    RULE_BASIC_CAVE, // NEW: Import ruleset constants
    RULE_MAZE
};

// --- EXTERNAL CRATE DEPENDENCIES ---
use aetherion_shared::chunk_data::ChunkData;
use aetherion_tools::get_config; // NEW: Import configuration utility

// FIX: Define a type alias that includes Send + Sync bounds for thread-safe trait objects
type DynGenerator = Box<dyn Generator + Send + Sync>;

// -----------------------------------------------------------------------------
// CONDUCTOR STATE AND STATUS (For Signal Inspector)
// -----------------------------------------------------------------------------

/// Represents the operational state of the Conductor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
    Initializing,
    Running,
    Paused,
    ShuttingDown,
    Error,
}

/// Shared, thread-safe state exposed to the CLI for monitoring.
#[derive(Clone)]
pub struct ConductorState {
    status: Arc<Mutex<ConductorStatus>>,
    // Represents the number of pending tasks in the generation queue
    queue_depth: Arc<AtomicUsize>, 
    // The ID of the currently active generation algorithm
    active_generator_id: Arc<Mutex<String>>,
}

impl ConductorState {
    pub fn new(initial_generator_id: String) -> Self {
        ConductorState {
            status: Arc::new(Mutex::new(ConductorStatus::Initializing)),
            queue_depth: Arc::new(AtomicUsize::new(0)),
            active_generator_id: Arc::new(Mutex::new(initial_generator_id)),
        }
    }

    // Public methods for the CLI to inspect the state
    pub fn get_status(&self) -> ConductorStatus {
        // Handle Mutex poisoning gracefully
        match self.status.lock() {
            Ok(guard) => *guard,
            Err(e) => {
                error!("Mutex poisoned when reading status: {}", e);
                ConductorStatus::Error
            }
        }
    }

    pub fn get_queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Relaxed)
    }

    pub fn get_active_generator_id(&self) -> String {
        // Handle Mutex poisoning by unwrapping (less critical than status)
        self.active_generator_id.lock().unwrap().clone()
    }
    
    // Internal methods for the Conductor to update the state
    pub(crate) fn set_status(&self, new_status: ConductorStatus) {
        *self.status.lock().unwrap() = new_status;
    }
    
    pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.lock().unwrap() = id.to_string();
    }
}

// -----------------------------------------------------------------------------
// CONDUCTOR MANAGER
// -----------------------------------------------------------------------------

/// The central manager for the procedural generation pipeline.
pub struct Conductor {
    runtime: Runtime,
    generators: HashMap<String, Arc<DynGenerator>>,
    internal_state: ConductorState,
    // PHASE 6: Thread-safe handle to the ChunkCache for persistence
    chunk_cache: Arc<Mutex<ChunkCache>>,
}
            
impl Conductor {
    /// Initializes the Conductor, starts the runtime, and returns a thread-safe
    /// copy of its state for external monitoring.
    pub fn new() -> Result<(Self, ConductorState), io::Error> {
        // Load configuration first
        let config = get_config(); 

        // --- Runtime Setup ---
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4) 
            .enable_all()
            .build()?;

        // --- Generator Registration ---
        let mut generators: HashMap<String, Arc<DynGenerator>> = HashMap::new();
        
        // 1. Perlin Generator (MVG) - Note: Scale 64.0 is now the assumed default
        let perlin: DynGenerator = Box::new(PerlinGenerator::new(64.0));
        let default_perlin_id = perlin.id().to_string();
        generators.insert(default_perlin_id.clone(), Arc::new(perlin));

        // 2. Cellular Automata Generator (Cave)
        let ca_cave: DynGenerator = Box::new(CellularAutomataGenerator::new(RULE_BASIC_CAVE));
        generators.insert(ca_cave.id().to_string(), Arc::new(ca_cave));
        
        // 3. Cellular Automata Generator (Maze)
        let ca_maze: DynGenerator = Box::new(CellularAutomataGenerator::new(RULE_MAZE));
        generators.insert(ca_maze.id().to_string(), Arc::new(ca_maze));
        
        // --- Cache Initialization (PHASE 6) ---
        let chunk_cache = match ChunkCache::new() {
            Ok(c) => {
                info!("ChunkCache initialized successfully.");
                Arc::new(Mutex::new(c))
            },
            Err(e) => {
                error!("Failed to initialize ChunkCache: {:?}", e);
                return Err(io::Error::new(io::ErrorKind::Other, "Cache initialization failed"));
            }
        };

        // --- Conductor State Initialization ---
        let mut initial_id = default_perlin_id.clone();
        
        // Use configured default, falling back to Perlin if not found
        let config_id = config.get_default_generator_id();
        if generators.contains_key(config_id) {
            initial_id = config_id.to_string();
        } else {
            warn!("Config default generator ID '{}' not found. Falling back to Perlin: {}", config_id, default_perlin_id);
        }

        let state = ConductorState::new(initial_id.clone());
        state.set_status(ConductorStatus::Running);
        
        info!("Conductor initialized. Active generator: {}", initial_id);

        Ok((Conductor {
            runtime,
            generators,
            internal_state: state.clone(),
            chunk_cache, // Store the initialized cache
        }, state)) // Return the state clone for the CLI
    }

    /// Public method to get a handle to the Runtime for task spawning.
    pub fn get_handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// Provides public read-access to the active generator ID.
    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }
    
    // --- Shutdown Management (The Fix) ---

    /// Signals the Conductor's internal state to begin a graceful shutdown process.
    /// This method is non-consuming (`&self`) and primarily updates the shared state,
    /// which is suitable for FFI access where the Conductor singleton must remain.
    pub fn signal_shutdown_graceful(&self) {
        info!("Aetherion Conductor signaled for shutdown. Setting status to ShuttingDown.");
        self.internal_state.set_status(ConductorStatus::ShuttingDown);
    }
    
    /// Performs a full, graceful teardown of the Conductor, signaling shutdown and
    /// stopping the underlying Tokio runtime. This method consumes `self`.
    pub fn graceful_teardown(self) {
        self.signal_shutdown_graceful();
        
        // This stops the Tokio runtime pool, freeing its resources.
        self.runtime.shutdown_background(); 
        info!("Aetherion Conductor full teardown complete.");
    }
    
    // NOTE: The previous `pub fn shutdown(self)` has been replaced by `graceful_teardown(self)`
    // and `signal_shutdown_graceful(&self)`.

    // --- Generator Management & Core Pipeline ---

    /// Changes the algorithm used for subsequent generation tasks.
    pub fn set_active_generator(&mut self, id: &str) -> Result<(), String> {
        if self.generators.contains_key(id) {
            info!("Active generator set to: {}", id);
            self.internal_state.set_active_generator_id(id);
            Ok(())
        } else {
            let err = format!("Generator ID '{}' not found. Available IDs: {:?}", id, self.generators.keys());
            error!("{}", err);
            Err(err)
        }
    }
    
    /// The primary synchronous function used to generate a chunk (blocking for now).
    ///
    /// PHASE 6: Implements **Cache-Load -> Generate -> Cache-Save** logic.
    pub fn generate_single_chunk(&self, chunk_coords: Vec2i) -> ChunkData {
        // Convert 2D chunk coordinates to a 3D ChunkKey (assuming Z=0)
        let key_vec3 = IVec3::new(chunk_coords.x, chunk_coords.y, 0);
        let chunk_key = ChunkKey(key_vec3);

        // --- 1. Attempt to load from cache ---
        match self.chunk_cache.lock() {
            Ok(cache_lock) => {
                match cache_lock.load_chunk(&chunk_key) {
                    Ok(Some(data)) => {
                        info!("Conductor retrieved chunk {:?} (Key: {:?}) from cache.", chunk_coords, chunk_key);
                        return data;
                    },
                    Ok(None) => {
                        // Not in cache, proceed to generation.
                        info!("Chunk {:?} not found in cache. Generating...", chunk_coords);
                    },
                    Err(e) => {
                        warn!("Cache load failed for {:?}: {:?}. Generating instead.", chunk_coords, e);
                    }
                }
            },
            Err(e) => {
                error!("Cache Mutex poisoned during load: {}", e);
            }
        }
        
        // --- 2. Generate the Chunk ---
        let active_id = self.internal_state.get_active_generator_id();
        let generator_arc = self.generators
            .get(&active_id)
            .expect("Active generator ID must be registered in Conductor.");
            
        info!("Conductor dispatching generation of chunk {:?} using '{}'", 
              chunk_coords, active_id);
            
        let chunk_data = generator_arc.generate_chunk(chunk_coords);
        
        // --- 3. Save to cache ---
        match self.chunk_cache.lock() {
            Ok(cache_lock) => {
                if let Err(e) = cache_lock.save_chunk(&chunk_key, &chunk_data) {
                    error!("Failed to save chunk {:?} (Key: {:?}) to cache: {:?}", chunk_coords, chunk_key, e);
                } else {
                    info!("Conductor saved chunk {:?} to cache.", chunk_coords);
                }
            },
            Err(e) => {
                error!("Cache Mutex poisoned during save: {}", e);
            }
        }
        
        chunk_data
    }
}