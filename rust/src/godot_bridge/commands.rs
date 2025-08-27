use godot::prelude::*;
use godot::classes::TileMap;

use crate::core::map_builder::MapBuilder;
use crate::data_processing::level_importer;
use crate::godot_bridge::signals::AetherionSignals;
use crate::utilities::concurrency::{self, TilePlacementMessage};

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct AetherionEngine {
    base: Base<Node>,

    tilemap: Option<Gd<TileMap>>,
    signals_node: Option<Gd<AetherionSignals>>,
    receiver: Option<concurrency::Receiver<TilePlacementMessage>>,
}

#[godot_api]
impl AetherionEngine {
    #[func]
    fn _ready(&mut self) {
        godot_print!("🧠 AetherionEngine is ready.");
    }

    #[func]
    pub fn set_signals_node(&mut self, signals: Gd<AetherionSignals>) {
        self.signals_node = Some(signals);
        godot_print!("🔗 Signals node connected.");
    }

    #[func]
    pub fn build_map(&mut self, tilemap: Gd<TileMap>) {
        self.tilemap = Some(tilemap.clone());

        let Some(mut signals_node) = self.signals_node.clone() else {
            godot_error!("❌ Cannot build map: signals node is not set.");
            return;
        };

        let mut map_builder = MapBuilder::new(Vector2i::new(100, 100), 12345);
        map_builder.build_map(signals_node);

        match level_importer::load_map_data() {
            Ok(map_data) => {
                self.receiver = Some(concurrency::spawn_tile_placement_task(map_data, tilemap));
                godot_print!("🧬 Map data loaded and async task spawned.");
            }
            Err(e) => {
                godot_error!("❌ Map loading failed: {}", e);
            }
        }
    }

    #[func]
    fn _process(&mut self, _delta: f64) {
        if let Some(rx) = &self.receiver {
            while let Ok(msg) = rx.try_recv() {
                match &mut self.tilemap {
                    Some(tilemap) => {
                        let coords = msg.position;
                        let info = msg.info;
                        let layer = 0;

                        tilemap.set_cell_ex(layer, coords)
                            .source_id(info.source_id)
                            .atlas_coords(Vector2i::new(info.atlas_coords.x, info.atlas_coords.y))
                            .alternative_tile(info.alternate_id)
                            .done();
                    }
                    None => {
                        godot_error!("⚠️ Tilemap reference missing during _process.");
                    }
                }
            }
        }
    }

    #[func]
	pub fn bulk_place_tiles(&self, mut tilemap: Gd<TileMap>, black: Vector2i, blue: Vector2i) {
		use rand::Rng;
	
		let grid_size = 1732; // ~3 million tiles
		let layer = 0;
		let mut rng = rand::thread_rng();

		let start = std::time::Instant::now();

		for y in 0..grid_size {
			for x in 0..grid_size {
				let pos = Vector2i::new(x, y);
				let atlas = if rng.gen_bool(0.5) { black } else { blue };

				tilemap.set_cell_ex(layer, pos)
					.source_id(0)
					.atlas_coords(atlas)
					.alternative_tile(0)
					.done();
			}
		}

		let elapsed = start.elapsed();
		godot_print!(
			"⏱️ Randomized 3 million tiles in {:.3} seconds.",
			elapsed.as_secs_f64()
		);
	}

}
