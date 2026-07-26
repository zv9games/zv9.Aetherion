//! Aetherion Conductor — The Async Orchestrator of Infinite Worlds
//!
//! Coordinates procedural generation, caching, and real-time feedback.
//! Powered by Tokio. Built for the hopeless wanderers.

use tokio::runtime::{Runtime, Handle};
use tokio::sync::mpsc;
use tracing::{info, error, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::io;

use aetherion_cache::ChunkCache;
use aetherion_math::{Vec2i, prelude::ChunkKey};
use glam::IVec3;
use aetherion_shared::chunk_data::ChunkData;
use aetherion_tools::get_config_from_path;

use crate::Generator;
use crate::perlin_generator::PerlinGenerator;
use crate::cellular_automata_generator::{
    CellularAutomataGenerator,
    RULE_BASIC_CAVE, RULE_MAZE, RULE_SOLID, RULE_CHECKERBOARD,
};

// ── Core Types ──────────────────────────────────────────────────────────────
type DynGenerator = Box<dyn Generator + Send + Sync>;
const PROGRESS_CHANNEL_BOUND: usize = 100;

#[derive(Debug)]
pub enum GenerationMessage {
    StatusUpdate(String),
    ChunkGenerated(Vec2i, ChunkData),
    GenerationComplete,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub width: usize,
    pub height: usize,
    pub seed: String,
    pub generator_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConductorStatus {
    Initializing,
    Running,
    Paused,
    ShuttingDown,
    Error,
}

// ── Shared State (Thread-Safe Monitoring) ───────────────────────────────────
#[derive(Clone)]
pub struct ConductorState {
    status: Arc<Mutex<ConductorStatus>>,
    queue_depth: Arc<AtomicUsize>,
    active_generator_id: Arc<Mutex<String>>,
}

impl ConductorState {
    pub fn new(initial_id: String) -> Self {
        Self {
            status: Arc::new(Mutex::new(ConductorStatus::Initializing)),
            queue_depth: Arc::new(AtomicUsize::new(0)),
            active_generator_id: Arc::new(Mutex::new(initial_id)),
        }
    }

    pub fn get_status(&self) -> ConductorStatus {
        self.status
            .lock()
            .map(|guard| *guard)
            .unwrap_or_else(|e| {
                error!("ConductorState mutex poisoned: {}", e);
                ConductorStatus::Error
            })
    }

    pub fn get_queue_depth(&self) -> usize {
        self.queue_depth.load(Ordering::Relaxed)
    }

    pub fn get_active_generator_id(&self) -> String {
        self.active_generator_id.lock().unwrap().clone()
    }

    // Internal setters — only used by Conductor
    pub(crate) fn set_status(&self, status: ConductorStatus) {
        *self.status.lock().unwrap() = status;
    }

    pub(crate) fn set_active_generator_id(&self, id: &str) {
        *self.active_generator_id.lock().unwrap() = id.to_string();
    }
}

// ── Conductor: The Async Engine Core ────────────────────────────────────────
pub struct Conductor {
    runtime: Runtime,
    generators: HashMap<String, Arc<DynGenerator>>,
    internal_state: ConductorState,
    chunk_cache: Arc<Mutex<ChunkCache>>,
    progress_sender: mpsc::Sender<GenerationMessage>,
}

impl Conductor {
    /// Initializes the Conductor with config, runtime, generators, and cache.
    pub fn new(config_path: Option<&str>) -> Result<(Self, ConductorState, mpsc::Receiver<GenerationMessage>), io::Error> {
        let config = get_config_from_path(config_path)?;
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()?;

        let mut generators: HashMap<String, Arc<DynGenerator>> = HashMap::new();

        // Register Perlin
        let perlin = Box::new(PerlinGenerator::new(64.0));
        let default_id = perlin.id().to_string();
        generators.insert(default_id.clone(), Arc::new(perlin));

        // Register Cellular Automata variants
        macro_rules! register_ca {
            ($rule:expr) => {{
                let gen = Box::new(CellularAutomataGenerator::new($rule));
                generators.insert(gen.id().to_string(), Arc::new(gen));
            }};
        }
        register_ca!(RULE_BASIC_CAVE);
        register_ca!(RULE_MAZE);
        register_ca!(RULE_SOLID);
        register_ca!(RULE_CHECKERBOARD);

        // Initialize cache
        let chunk_cache = ChunkCache::new()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let chunk_cache = Arc::new(Mutex::new(chunk_cache));

        // MPSC channel for progress
        let (progress_sender, progress_receiver) = mpsc::channel(PROGRESS_CHANNEL_BOUND);

        // Determine initial generator
        let default_gen_id = config.get_default_generator_id();
        let initial_id = if generators.contains_key(default_gen_id) {
            default_gen_id.to_string()
        } else {
            default_id
        };

        let state = ConductorState::new(initial_id.clone());
        state.set_status(ConductorStatus::Running);
        info!("Conductor initialized. Active generator: {}", initial_id);

        Ok((
            Self {
                runtime,
                generators,
                internal_state: state.clone(),
                chunk_cache,
                progress_sender,
            },
            state,
            progress_receiver,
        ))
    }

    // ── Public API ──────────────────────────────────────────────────────────
    pub fn get_handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    pub fn get_active_generator_id(&self) -> String {
        self.internal_state.get_active_generator_id()
    }

    /// Starts async generation of the full map.
    pub fn start_generation(&mut self, config: GeneratorConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.set_active_generator(&config.generator_name)?;

        let state = self.internal_state.clone();
        let sender = self.progress_sender.clone();
        let generators = self.generators.clone();
        let cache = self.chunk_cache.clone();
        let active_id = self.internal_state.get_active_generator_id();

        self.runtime.spawn(async move {
            let _ = sender.send(GenerationMessage::StatusUpdate("Starting generation...".into())).await;
            state.queue_depth.fetch_add(1, Ordering::Relaxed);

            let chunk_size = ChunkData::SIZE as usize;
            let chunks_x = (config.width + chunk_size - 1) / chunk_size;
            let chunks_y = (config.height + chunk_size - 1) / chunk_size;

            if chunks_x == 0 || chunks_y == 0 {
                let _ = sender.send(GenerationMessage::Error("Map dimensions too small — zero chunks.".into())).await;
                state.queue_depth.fetch_sub(1, Ordering::Relaxed);
                return;
            }

            for y in 0..chunks_y {
                for x in 0..chunks_x {
                    let coords = Vec2i::new(x as i32, y as i32);
                    let key = ChunkKey(IVec3::new(coords.x, coords.y, 0));

                    let data = match cache.lock() {
                        Ok(c) => c
                            .load_chunk(&key)
                            .ok()
                            .flatten()
                            .unwrap_or_else(|| generators[&active_id].generate_chunk(coords)),
                        Err(e) => {
                            error!("Cache mutex poisoned: {}. Forcing generation.", e);
                            generators[&active_id].generate_chunk(coords)
                        }
                    };

                    if sender.send(GenerationMessage::ChunkGenerated(coords, data)).await.is_err() {
                        state.queue_depth.fetch_sub(1, Ordering::Relaxed);
                        return;
                    }
                }
            }

            let _ = sender.send(GenerationMessage::GenerationComplete).await;
            state.queue_depth.fetch_sub(1, Ordering::Relaxed);
            info!("Generation task completed.");
        });

        Ok(())
    }

    /// Graceful shutdown signal
    pub fn signal_shutdown_graceful(&self) {
        self.internal_state.set_status(ConductorStatus::ShuttingDown);
    }

    /// Consumes self and shuts down runtime
    pub fn graceful_teardown(self) {
        self.signal_shutdown_graceful();
        self.runtime.shutdown_background();
        info!("Conductor runtime terminated.");
    }

    /// Switches active generator
    pub fn set_active_generator(&mut self, id: &str) -> Result<(), String> {
        if self.generators.contains_key(id) {
            self.internal_state.set_active_generator_id(id);
            Ok(())
        } else {
            let available: Vec<_> = self.generators.keys().cloned().collect();
            Err(format!("Generator '{}' not found. Available: {:?}", id, available))
        }
    }

    /// Synchronous single-chunk generation (CLI, debug, FFI)
    pub fn generate_single_chunk(&self, coords: Vec2i) -> ChunkData {
        let key = ChunkKey(IVec3::new(coords.x, coords.y, 0));
        let active_id = self.internal_state.get_active_generator_id();
        let generator = &self.generators[&active_id];

        // Try cache
        if let Ok(mut cache) = self.chunk_cache.lock() {
            if let Ok(Some(data)) = cache.load_chunk(&key) {
                info!("Cache hit: {:?}", coords);
                return data;
            }
        }

        // Generate
        let data = generator.generate_chunk(coords);
        info!("Generated chunk: {:?}", coords);

        // Save to cache
        if let Ok(mut cache) = self.chunk_cache.lock() {
            let _ = cache.save_chunk(&key, &data);
        }

        data
    }
}