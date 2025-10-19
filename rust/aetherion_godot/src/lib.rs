// -------------------------------------------------------------------------------------------------
// AETHERION ENGINE: GODOT BINDINGS (aetherion_godot)
// -------------------------------------------------------------------------------------------------

// Explicitly import core types and traits
use godot::prelude::*; // Provides the majority of required types (GString, etc.)
use godot::classes::Node;
use godot::obj::Base;
use godot::builtin::GString;

// --- GDEXTENSION ENTRY POINT IMPORTS ---
// We only need ExtensionLibrary for the implementation below.
use godot::init::ExtensionLibrary; 

// Internal Crate Dependencies
use aetherion_generate::Conductor; 
use aetherion_math::Vec2i;

// Standard library and utilities
use std::sync::{Arc, Mutex};
use tracing::{info, error};

// --- GDEXTENSION ENTRY POINT ---
// This pattern automatically generates the FFI entry functions (like gdext_rust_init)
// and handles class registration based on the #[derive(GodotClass)] below.

struct AetherionExtension; 

#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {}

// -------------------------------------------------------------------------------------------------
// AETHERION ENGINE GODOT WRAPPER
// -------------------------------------------------------------------------------------------------

/// The Godot-facing wrapper class for the Aetherion Engine.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct AetherionEngine {
    /// The core Rust logic manager, safely wrapped in an Arc/Mutex for shared access.
    conductor: Option<Arc<Mutex<Conductor>>>,
    
    #[base]
    base: Base<Node>,
}

// ------------------------------------------------------------------------------------
// Constructor
// ------------------------------------------------------------------------------------
impl AetherionEngine {
    // --- Constructor ---
    pub fn init(base: Base<Node>) -> Self {
        info!("AetherionEngine: Initializing GDExtension Class.");
        
        let conductor = match Conductor::new() {
            Ok(c) => {
                info!("Aetherion Conductor initialized successfully.");
                Some(Arc::new(Mutex::new(c)))
            },
            Err(e) => {
                error!("Aetherion Conductor failed to initialize: {:?}", e);
                None
            }
        };

        Self {
            conductor,
            base,
        }
    }
}

// ------------------------------------------------------------------------------------
// API methods remain in the #[godot_api] block.
// ------------------------------------------------------------------------------------
#[godot_api]
impl AetherionEngine {
    // --- Public API Surface (Validation Target) ---
    
    /// Generates a single chunk synchronously using the currently active generator.
    #[func]
    pub fn generate_chunk_sync(&mut self, x: i32, y: i32) -> bool {
        let chunk_coords = Vec2i::new(x, y);

        let Some(conductor_arc) = self.conductor.as_ref() else {
            error!("Cannot generate chunk: Conductor not initialized.");
            return false;
        };

        match conductor_arc.lock() {
            Ok(conductor) => {
                info!("Godot: Calling core generate_single_chunk for {:?}", chunk_coords);
                conductor.generate_single_chunk(chunk_coords);
                true
            },
            Err(e) => {
                error!("Mutex lock failed during chunk generation: {:?}", e);
                false
            }
        }
    }

    /// Sets the active generator algorithm by its string ID (e.g., "perlin_basic_2d").
    #[func]
    pub fn set_generator(&mut self, id: GString) -> bool {
        let id_str = id.to_string();
        
        let Some(conductor_arc) = self.conductor.as_ref() else {
            error!("Cannot set generator: Conductor not initialized.");
            return false;
        };

        match conductor_arc.lock() {
            Ok(mut conductor) => {
                conductor.set_active_generator(&id_str).is_ok()
            },
            Err(e) => {
                error!("Mutex lock failed during set_generator: {:?}", e);
                false
            }
        }
    }
    
    /// Returns the ID of the currently active generator.
    #[func]
    pub fn get_active_generator_id(&self) -> GString {
        let Some(conductor_arc) = self.conductor.as_ref() else {
            return GString::from("ERROR: CONSTRUCTOR FAILED");
        };

        match conductor_arc.lock() {
            Ok(conductor) => GString::from(conductor.get_active_generator_id()), 
            Err(_) => GString::from("ERROR: MUTEX POISONED"),
        }
    }

    // --- Destructor Replacement ---
    /// Expose a function for Godot to call on cleanup.
    #[func]
    pub fn shutdown_engine(&mut self) {
        info!("AetherionEngine: Shutting down.");
        
        if let Some(conductor_arc) = self.conductor.take() {
            if let Ok(c) = Arc::try_unwrap(conductor_arc) {
                if let Ok(conductor) = c.into_inner() {
                    conductor.shutdown();
                }
            } else {
                error!("AetherionEngine: Cannot fully shutdown Conductor; other references still exist.");
            }
        }
    }
}