use godot::prelude::*;
use godot_macros::GodotClass;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::distributions::{Uniform, Distribution};
use crate::api_bot::APIBot;

#[derive(GodotClass, Debug)]
#[class(init, base=Node)]
pub struct ChangeOver {
    #[base]
    base: Base<Node>,

    #[var]
    pub grid_width: i32,

    #[var]
    pub grid_height: i32,

    #[var]
    pub tile_layer: i32,

    #[var]
    pub seed: i64,

    #[var]
    pub flipper_active: bool,
}
// ✅ Godot lifecycle block
#[godot_api]
impl ChangeOver {
    fn new(base: Base<Node>) -> Self {
        Self {
            base,
            grid_width: 32,
            grid_height: 32,
            tile_layer: 0,
            seed: 123456789,
            flipper_active: false,
        }
    }
	
    /// 🧑‍🏫 Lifecycle hook — called when the node enters the scene tree
    #[func]
    fn _ready(&mut self) {
        godot_print!("🎓 ChangeOver is ready to flip tiles.");
    }

    /// 🎮 Ritual ignition
    #[func]
    pub fn mode_start(&mut self) {
        godot_print!("🧭 ChangeOver: mode_start() invoked.");
        godot_print!("🔁 Flipper activated.");
        self.flipper_active = true;
    }

    /// 🛑 Ritual halt
    #[func]
    pub fn mode_loaded(&mut self) {
        godot_print!("🌫️ ChangeOver: mode_loaded() invoked.");
        godot_print!("🛑 Flipper deactivated.");
        self.flipper_active = false;
    }

    /// 🔁 Internal flipper pulse
    #[func]
    pub fn tick(&self) {
        if !self.flipper_active {
            godot_print!("⏸️ Tick skipped: flipper inactive.");
            return;
        }

        godot_print!("🔁 Tick invoked: flipping 5 tiles...");
        let updates = self.flip_tiles(5);
        godot_print!("🔁 Tick generated {} updates.", updates.len());
        self.base.to_gd().call_deferred("apply_updates", &[Variant::from(updates)]);
    }

    /// 🎲 Generate a single random tile update
    #[func]
    pub fn random_tile(&self) -> Dictionary {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        let dist_x = Uniform::new(0, self.grid_width);
        let dist_y = Uniform::new(0, self.grid_height);
        let dist_id = Uniform::new(0, 2);

        let x = dist_x.sample(&mut rng);
        let y = dist_y.sample(&mut rng);
        let tile_id = dist_id.sample(&mut rng);

        godot_print!("🎲 Random tile: pos=({}, {}), id={}", x, y, tile_id);

        let mut tuple = Dictionary::new();
        tuple.insert("pos", Vector2i::new(x, y));
        tuple.insert("layer", self.tile_layer);
        tuple.insert("tile_id", tile_id);
        tuple
    }

    /// 🔁 Generate multiple random tile updates
    #[func]
    pub fn flip_tiles(&self, count: i32) -> VariantArray {
        godot_print!("🔁 flip_tiles({}) invoked.", count);
        let mut updates = VariantArray::new();
        for i in 0..count {
            let tile = self.random_tile();
            godot_print!("🔁 [{}] {:?}", i, tile);
            updates.push(&Variant::from(tile));
        }
        updates
    }

    /// 🌫️ Dissolve tiles into nil
    #[func]
    pub fn dissolve_tiles(&self, tiles: Array<Variant>) -> VariantArray {
        godot_print!("🌫️ dissolve_tiles() invoked. Tile count: {}", tiles.len());
        let mut frame = VariantArray::new();
        for (i, tile) in tiles.iter_shared().enumerate() {
            if let Ok(dict) = tile.try_to::<Dictionary>() {
                let pos = dict.get("pos").unwrap_or(Variant::nil());
                godot_print!("🌫️ [{}] Dissolving tile at {:?}", i, pos);
                let mut tuple = Dictionary::new();
                tuple.insert("pos", pos);
                tuple.insert("layer", self.tile_layer);
                tuple.insert("tile_id", Variant::nil());
                frame.push(&Variant::from(tuple));
            } else {
                godot_print!("⚠️ [{}] Invalid tile format.", i);
            }
        }
        frame
    }

    /// 🖤 Fill grid with black veil
    #[func]
    pub fn fill_black(&self) -> VariantArray {
        godot_print!("🖤 fill_black() invoked. Grid: {}x{}", self.grid_width, self.grid_height);
        let mut updates = VariantArray::new();
        for x in 0..self.grid_width {
            for y in 0..self.grid_height {
                let mut tuple = Dictionary::new();
                tuple.insert("pos", Vector2i::new(x, y));
                tuple.insert("layer", self.tile_layer);
                tuple.insert("tile_id", 0); // Black tile ID
                updates.push(&Variant::from(tuple));
            }
        }
        godot_print!("🖤 Veil applied to {} tiles.", updates.len());
        updates
    }
}
// ✅ Trait logic — outside macro scope
#[godot_api]
impl INode for ChangeOver {}
