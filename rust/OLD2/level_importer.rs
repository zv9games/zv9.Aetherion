use godot::prelude::*;
use godot::classes::{FileAccess, ResourceLoader, Image as GodotImage, Texture2D};
use godot::classes::file_access::ModeFlags;
use std::collections::HashMap;
use bincode::{serialize, deserialize};

use crate::data_processing::types::{MapDataChunk, TileInfo, SerializableVector2i};

const CACHE_PATH: &str = "user://expansive_tile_data.cache";
const IMAGE_PATH: &str = "res://PACMAN/ASSETS/maps/world.png";

const TILE_MAP_CONST: [(&str, SerializableVector2i); 2] = [
    ("wall", SerializableVector2i { x: 15, y: 13 }),
    ("floor", SerializableVector2i { x: 14, y: 13 }),
];

const THRESHOLD_WHITE: f32 = 0.8;
const THRESHOLD_BLACK: f32 = 0.2;
const FORCE_TILE_REGEN: bool = false;

pub fn load_map_data() -> Result<MapDataChunk, String> {
    if !FORCE_TILE_REGEN && FileAccess::file_exists(CACHE_PATH) {
        if let Some(file_access) = FileAccess::open(CACHE_PATH, ModeFlags::READ) {
            let packed = file_access.get_buffer(file_access.get_length() as i64);
            let raw: &[u8] = packed.as_slice();
            if let Ok(map_data) = deserialize::<MapDataChunk>(raw) {
                godot_print!("💾 Loaded tile data from cache.");
                return Ok(map_data);
            }
        }
    }

    let map_data = process_image()?;

    if let Some(mut file_access) = FileAccess::open(CACHE_PATH, ModeFlags::WRITE) {
        if let Ok(serialized) = serialize(&map_data) {
            let packed = PackedByteArray::from(serialized);
            file_access.store_buffer(&packed);
            godot_print!("📝 Cached tile data to disk.");
        }
    }

    Ok(map_data)
}

fn process_image() -> Result<MapDataChunk, String> {
    let mut loader = ResourceLoader::singleton();
    let texture = loader.load(IMAGE_PATH)
        .ok_or_else(|| "❌ Failed to load image texture from path.".to_string())?;

    let texture = texture.try_cast::<Texture2D>()
        .map_err(|_| "❌ Loaded resource is not a Texture2D.".to_string())?;

    let image = texture.get_image()
        .ok_or_else(|| "❌ Failed to get Godot Image from texture.".to_string())?;

    let width = image.get_width();
    let height = image.get_height();
    let mut tile_data: MapDataChunk = HashMap::new();

    let half_width = width / 2;
    let half_height = height / 2;

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixelv(Vector2i::new(x, y));

            if pixel.a < 0.5 {
                continue;
            }

            let lum = pixel.r * 0.2126 + pixel.g * 0.7152 + pixel.b * 0.0722;
            let label = match lum {
                l if l > THRESHOLD_WHITE => Some("wall"),
                l if l < THRESHOLD_BLACK => Some("floor"),
                _ => None,
            };

            if let Some(tile_label) = label {
                if let Some(coords) = TILE_MAP_CONST.iter()
                    .find(|(name, _)| *name == tile_label)
                    .map(|(_, coords)| coords.clone()) // clone to avoid move
                {
                    let pos = SerializableVector2i {
                        x: x - half_width,
                        y: y - half_height,
                    };
                    tile_data.insert(pos, TileInfo {
                        source_id: 0,
                        atlas_coords: coords,
                        alternate_id: 0,
                    });
                }
            }
        }
    }

    godot_print!("🧮 Image processed into tile data.");
    Ok(tile_data)
}
