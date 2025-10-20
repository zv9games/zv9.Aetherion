// -------------------------------------------------------------------------------------------------
// AETHERION ENGINE: GODOT BINDINGS (aetherion_godot)
// -------------------------------------------------------------------------------------------------

// Explicitly import core types and traits
use godot::prelude::*; // Provides GString, Dictionary, Array, Variant, godot_error, etc.
use godot::classes::Node;
use godot::obj::Base;
use godot::builtin::GString;

// --- GDEXTENSION ENTRY POINT IMPORTS ---
use godot::init::ExtensionLibrary; 

// Internal Crate Dependencies
use aetherion_generate::Conductor; 
use aetherion_math::Vec2i;

// Standard library and utilities
use std::sync::{Arc, Mutex};
use tracing::info; // Keep info for internal Rust logging
use godot::prelude::godot_error; // Use the Godot logging macro for errors

// --- GDEXTENSION ENTRY POINT ---

struct AetherionExtension; 

#[gdextension(entry_symbol = gdext_rust_init)]
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
        // Minimal constructor; lazy init moved to methods for reliable logging
        Self {
            conductor: None,
            base,
        }
    }
}

// ------------------------------------------------------------------------------------
// API methods remain in the #[godot_api] block.
// ------------------------------------------------------------------------------------
#[godot_api]
impl AetherionEngine {
    // Helper to initialize conductor lazily with logging
    fn ensure_conductor(&mut self) -> bool {
        if self.conductor.is_some() {
            return true;
        }

        godot_error!("--- Initializing Conductor lazily ---");
        match Conductor::new(None) {
            Ok((conductor_instance, _state)) => {
                info!("Aetherion Conductor initialized successfully.");
                self.conductor = Some(Arc::new(Mutex::new(conductor_instance)));
                true
            }
            Err(e) => {
                godot_error!("Aetherion Conductor failed to initialize: REASON: {:?}", e);
                false
            }
        }
    }

    // --- Public API Surface (Validation Target) ---
    
    /// Returns the generated chunk data as a Godot Dictionary, 
    /// satisfying the GDScript validation expectations.
    #[func]
    pub fn generate_chunk(&mut self, x: i32, y: i32, key_z: i32) -> Dictionary {
        let chunk_coords = Vec2i::new(x, y);
        let mut result_dict = Dictionary::new(); 

        if !self.ensure_conductor() {
            godot_error!("Cannot generate chunk: Conductor not initialized.");
            return result_dict;
        }

        let conductor_arc = self.conductor.as_ref().unwrap(); // Safe after ensure

        match conductor_arc.lock() {
            Ok(conductor) => {
                info!("Godot: Calling core generate_single_chunk for {:?}", chunk_coords);
                let chunk_data = conductor.generate_single_chunk(chunk_coords);
                
                // --- DATA CONVERSION: Rust `ChunkData` to Godot `Dictionary` ---
                let mut tile_array = Array::new();
                
                for tile in chunk_data.tiles {
                    let mut tile_dict = Dictionary::new();
                    
                    tile_dict.set("id", Variant::from(tile.tile_type as i32));
                    tile_dict.set("level", Variant::from(tile.noise_value));
                    
                    // Corrected: Use duplicate_deep() for deep copy, to_variant(), and pass reference
                    tile_array.push(&tile_dict.duplicate_deep().to_variant());
                }

                // Populate the result dictionary
                result_dict.set("key_x", Variant::from(x));
                result_dict.set("key_y", Variant::from(y));
                result_dict.set("key_z", Variant::from(key_z));
                result_dict.set("tile_count", Variant::from(tile_array.len() as i32));
                // Use to_variant() for Array when setting it in a Dictionary
                result_dict.set("tiles", tile_array.to_variant()); 
            },
            Err(e) => {
                godot_error!("Mutex lock failed during chunk generation: {:?}", e);
            }
        }
        
        result_dict
    }

    /// Sets the active generator algorithm by its string ID (e.g., "perlin_basic_2d").
    #[func]
    pub fn set_generator(&mut self, id: GString) -> bool {
        let id_str = id.to_string();
        
        if !self.ensure_conductor() {
            godot_error!("Cannot set generator: Conductor not initialized.");
            return false;
        }

        let conductor_arc = self.conductor.as_ref().unwrap(); // Safe after ensure

        match conductor_arc.lock() {
            Ok(mut conductor) => {
                conductor.set_active_generator(&id_str).is_ok()
            },
            Err(e) => {
                godot_error!("Mutex lock failed during set_generator: {:?}", e);
                false
            }
        }
    }
    
    /// Returns the ID of the currently active generator.
    #[func]
    pub fn get_active_generator_id(&self) -> GString {
        if self.conductor.is_none() {
            // Cannot lazy init here (immutable self); log and return error
            godot_error!("Cannot get active generator: Conductor not initialized.");
            return GString::from("ERROR: CONDUCTOR NOT INITIALIZED");
        }

        let conductor_arc = self.conductor.as_ref().unwrap();

        match conductor_arc.lock() {
            Ok(conductor) => GString::from(conductor.get_active_generator_id().as_str()), 
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
                    conductor.graceful_teardown();
                }
            } else {
                godot_error!("AetherionEngine: Cannot fully shutdown Conductor; other references still exist.");
            }
        }
    }
}