use godot::prelude::*;
use godot::classes::{FileAccess, ResourceLoader};
use std::collections::HashMap;

use crate::data_processing::types::MapDataChunk;

const CACHE_PATH: &str = "user://expansive_tile_data.cache";
const IMAGE_PATH: &str = "res://PACMAN/ASSETS/maps/world.png";

pub fn load_map_data() -> Result<MapDataChunk, String> {
    // Stage 1: Check for and load the cached data.

    // Inside the load_map_data() function
	if !FORCE_TILE_REGEN && FileAccess::file_exists(CACHE_PATH.into()) {
		// Open the file for reading
		if let Ok(file_access) = FileAccess::open(CACHE_PATH.into(), FileAccess::ModeFlags::READ) {
			// We'll read the entire file into a byte buffer
			let data = file_access.get_buffer(file_access.get_length() as i64);

			// Deserialize the data using bincode
			if let Ok(map_data) = bincode::deserialize::<MapDataChunk>(&data) {
				godot_print!("💾 Loaded tile data from cache.");
				return Ok(map_data);
			}
		}
	}

    // Stage 2: If cache is missing or regeneration is forced, process the image.

    // Your logic here...

    // Return the final data chunk.
    unimplemented!()
}
// In data_processing/level_importer.rs

use godot::prelude::*;
use godot::classes::{FileAccess, ResourceLoader, Image as GodotImage};
use std::collections::HashMap;

use crate::data_processing::types::{MapDataChunk, TileInfo};

// ... constants and `load_map_data` function from before ...

fn process_image() -> Result<MapDataChunk, String> {
    // Load the Texture2D from the `res://` path using Godot's loader
    let loader = ResourceLoader::singleton();
    let texture = loader.load(IMAGE_PATH.into())
        .ok_or("Failed to load image texture from path.".to_string())?
        .try_cast::<godot::classes::Texture2D>()
        .ok_or("Loaded resource is not a Texture2D.".to_string())?;

    // Get the Godot Image from the texture
    let image_godot = texture.get_image().ok_or("Failed to get Godot Image from texture.".to_string())?;

    let width = image_godot.get_width();
    let height = image_godot.get_height();
    let mut tile_data: MapDataChunk = HashMap::new();

    // Define the tile mappings as a simple constant for now.
    // This could later be loaded from a config file.
    const TILE_MAP_CONST: [(&str, Vector2i); 2] = [
        ("wall", Vector2i::new(15, 13)),
        ("floor", Vector2i::new(14, 13)),
    ];
    const THRESHOLD_WHITE: f32 = 0.8;
    const THRESHOLD_BLACK: f32 = 0.2;
    let half_width = width / 2;
    let half_height = height / 2;

    for y in 0..height {
        for x in 0..width {
            let pixel = image_godot.get_pixel_v(Vector2i::new(x, y));
            if pixel.a < 0.5 {
                continue; // Skip transparent pixels
            }
            
            // Replicate GDScript luminance calculation
            let lum = (pixel.r * 0.2126 + pixel.g * 0.7152 + pixel.b * 0.0722);
            
            let mut label = None;
            if lum > THRESHOLD_WHITE {
                label = Some("wall");
            } else if lum < THRESHOLD_BLACK {
                label = Some("floor");
            }
            
            if let Some(tile_label) = label {
                if let Some(coords) = TILE_MAP_CONST.iter().find(|(name, _)| *name == tile_label).map(|(_, coords)| *coords) {
                    let pos = Vector2i::new(x - half_width, y - half_height);
                    
                    tile_data.insert(
                        pos,
                        TileInfo {
                            source_id: 0,
                            atlas_coords: coords,
                            alternate_id: 0,
                        },
                    );
                }
            }
        }
    }
    
    Ok(tile_data)
}