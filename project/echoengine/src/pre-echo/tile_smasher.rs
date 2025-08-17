use godot::prelude::*;
use godot::builtin::{Vector2i, VariantArray, Dictionary};
use godot::classes::{TileMapLayer, Image};
use crate::threading::take_payload_batch;
use crate::types::{FLOOR_TILE, WALL_TILE};
use crate::logging::{log, LogLevel};
use thiserror::Error;
use crate::changeover::ChangeOver;

#[derive(Error, Debug)]
pub enum TileSmasherError {
    #[error("Invalid dictionary entry at key {0}: expected Dictionary, got {1}")]
    InvalidDictEntry(String, String),
    #[error("Null TileMapLayer instance at {0}")]
    NullTileMapLayer(String),
    #[error("Invalid tile data at position {0}")]
    InvalidTileData(Vector2i),
}

fn parse_tile_entry(entry: Variant, key: &str) -> Result<(i32, i32, i32, Vector2i, i32), TileSmasherError> {
    let dict = entry
        .try_to::<Dictionary>()
        .map_err(|e| TileSmasherError::InvalidDictEntry(key.to_string(), e.to_string()))?;

    let x = dict.get("x").and_then(|v| v.try_to::<i64>().ok()).unwrap_or_default() as i32;
    let y = dict.get("y").and_then(|v| v.try_to::<i64>().ok()).unwrap_or_default() as i32;
    let source_id = dict.get("source_id").and_then(|v| v.try_to::<i64>().ok()).unwrap_or_default() as i32;
    let atlas_coords = dict.get("atlas_coords").and_then(|v| v.try_to::<Vector2i>().ok()).unwrap_or(Vector2i::new(-1, -1));
    let alt_id = dict.get("alternate_id").and_then(|v| v.try_to::<i64>().ok()).unwrap_or_default() as i32;

    Ok((x, y, source_id, atlas_coords, alt_id))
}

#[derive(GodotClass, Debug)]
#[class(base=Node, init)]
pub struct TileSmasher {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl TileSmasher {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    /// 🧑‍🏫 Lifecycle hook — called when the node enters the scene tree
    #[func]
    fn _ready(&mut self) {
        log(LogLevel::Info, "🧱 TileSmasher is ready to smash.");
    }

    #[signal]
    fn tiles_smashed();

    #[func]
    fn smash_tiles_bulk(&mut self, mut tilemap: Gd<TileMapLayer>, payload: Dictionary) -> bool {
        if !tilemap.is_instance_valid() {
            log(LogLevel::Error, "🚫 TileMapLayer is null or invalid in smash_tiles_bulk");
            return false;
        }

        let mut updates = Vec::new();
        let mut success = true;

        for (key, entry) in payload.iter_shared() {
            let key_str = key.to_string();
            match parse_tile_entry(entry, &key_str) {
                Ok((x, y, source_id, atlas_coords, alt_id)) => {
                    let pos = Vector2i::new(x, y);
                    updates.push((pos, source_id, atlas_coords, alt_id));
                }
                Err(e) => {
                    log(LogLevel::Warn, &format!("⚠️ Skipping invalid entry at key {}: {}", key_str, e));
                    success = false;
                }
            }
        }

        for (pos, source_id, atlas_coords, alt_id) in updates.iter() {
            tilemap.set_cell(*pos);
            match tilemap.get_cell_tile_data(*pos) {
                Some(mut td) => {
                    td.set_custom_data("source_id", &source_id.to_variant());
                    td.set_custom_data("atlas_coords", &atlas_coords.to_variant());
                    td.set_custom_data("alt_id", &alt_id.to_variant());
                }
                None => {
                    log(LogLevel::Warn, &format!("🚫 No tile data found at {:?}", pos));
                    success = false;
                }
            }
        }

        log(LogLevel::Info, &format!("🔨 TileSmasher placed {} tiles.", updates.len()));
        self.base.to_gd().emit_signal("tiles_smashed", &[]);
        success
    }

    #[func]
    fn run_decoder(&mut self, image: Gd<Image>, start_x: i32, start_y: i32, chunk_size: i32) -> VariantArray {
        if !image.is_instance_valid() {
            log(LogLevel::Error, "🚫 Image instance is null in run_decoder");
            return VariantArray::new();
        }

        let width = image.get_width();
        let height = image.get_height();
        let end_x = (start_x + chunk_size).min(width);
        let end_y = (start_y + chunk_size).min(height);

        let mut output_vec = Vec::new();

        for y in start_y..end_y {
            for x in start_x..end_x {
                let px = image.get_pixel(x, y);
                if px.a < 0.5 {
                    continue;
                }

                let luminance = px.luminance();
                let label = match luminance {
                    lum if lum > 0.8 => "wall",
                    lum if lum < 0.2 => "floor",
                    _ => continue,
                };

                let pos = Vector2i::new(x - width / 2, y - height / 2);
                let tile_id = if label == "wall" { WALL_TILE } else { FLOOR_TILE };

                let entry = VariantArray::from_iter([
                    pos.x.to_variant(),
                    pos.y.to_variant(),
                    0u32.to_variant(),
                    tile_id.to_variant(),
                    0u32.to_variant(),
                    GString::from(label).to_variant(),
                ]);

                output_vec.push(entry.to_variant());
            }
        }

        let output = VariantArray::from_iter(output_vec);
        log(LogLevel::Info, &format!("🧠 Decoder produced {} entries.", output.len()));
        output
    }

    #[func]
    fn smash_payload_batch(&mut self, mut tilemap: Gd<TileMapLayer>) {
        if !tilemap.is_instance_valid() {
            log(LogLevel::Error, "🚫 TileMapLayer is null or invalid in smash_payload_batch");
            return;
        }

        let batch = take_payload_batch();
        if batch.is_empty() {
            return;
        }

        for payload in batch.iter() {
            tilemap.set_cell(payload.pos);
            if let Some(mut td) = tilemap.get_cell_tile_data(payload.pos) {
                td.set_custom_data("source_id", &payload.source_id.to_variant());
                td.set_custom_data("atlas_coords", &payload.atlas_coords.to_variant());
                td.set_custom_data("alt_id", &payload.alt_id.to_variant());
            } else {
                log(LogLevel::Warn, &format!("🚫 No tile data found at {:?}", payload.pos));
            }
        }

        log(LogLevel::Info, &format!("🔨 TileSmasher placed {} tiles.", batch.len()));
        self.base.to_gd().emit_signal("tiles_smashed", &[]);
    }
}
