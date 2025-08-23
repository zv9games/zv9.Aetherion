// In godot_bridge/commands.rs
use godot::prelude::*;
use godot::classes::TileMap;
use crate::data_processing::level_importer;
use crate::utilities::concurrency::{self, TilePlacementMessage};

#[godot::register]
#[derive(GodotClass)]
#[class(tool)]
pub struct AetherionEngine {
    receiver: Option<concurrency::Receiver<concurrency::TilePlacementMessage>>,
    #[base]
    base: Base<Node>,
}

#[godot::methods]
impl AetherionEngine {
    #[method]
    pub fn build_map(&mut self, tilemap: Gd<TileMap>) {
        // 1. Send the start signal
        self.base.emit_signal("build_map_start", &[]);

        // 2. Load the map data (this is the synchronous part)
        let map_data = match level_importer::load_map_data() {
            Ok(data) => data,
            Err(e) => {
                godot_error!("Map loading failed: {}", e);
                return;
            }
        };

        // 3. Spawn the async task and get the receiver
        self.receiver = Some(concurrency::spawn_tile_placement_task(map_data, tilemap));
    }

    #[method]
    fn _process(&mut self, _delta: f64) {
        if let Some(ref rx) = self.receiver {
            // Check for new messages from the worker thread without blocking
            while let Ok(msg) = rx.try_recv() {
                // Get the tilemap reference safely.
                // This is where we will call godot_api_functions directly.
                let mut tilemap = self.base.get_node_as::<TileMap>("my_tilemap");
                
                // Now we can safely place the tile on the main thread
                let tile_coords = msg.position;
                let info = msg.info;
                tilemap.set_cell(
                    Vector2i::new(tile_coords.x, tile_coords.y), 
                    info.source_id, 
                    info.atlas_coords, 
                    info.alternate_id
                );
            }
        }
    }
}