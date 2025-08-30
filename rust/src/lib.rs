pub mod godot4;
pub mod aetherion;

use godot::prelude::*;
use godot::classes::{TileMap, Node};
use godot::global::Error;
use godot_macros::gdextension;
use rayon::prelude::*;
use rand::{Rng, SeedableRng};
use crate::godot4::messaging::{GodotSync, EngineMessage};
use crate::godot4::signals::AetherionSignals;
use crate::aetherion::pipeline::data::{MapDataChunk, SerializableVector2i, TileInfo};
use godot4::api::engine::AetherionEngine;

struct AetherionExtension;

#[gdextension]
unsafe impl ExtensionLibrary for AetherionExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("🚀 Aetherion GDExtension initialized.");
        }
    }

    
}
