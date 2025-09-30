use godot::prelude::*;
use godot::builtin::{Vector2i, Dictionary};
use godot::classes::{Image, FileAccess, ResourceLoader, file_access::ModeFlags, image::Format};
use crate::changeover::ChangeOver;
use crate::logging::{log, LogLevel};


const IMAGE_PATH: &str = "res://PACMAN/ASSETS/maps/world.png";
const TILE_MAP: &[(&str, Vector2i)] = &[("wall", Vector2i::new(15, 13)), ("floor", Vector2i::new(14, 13))];
const THRESHOLD_WHITE: f64 = 0.8;
const THRESHOLD_BLACK: f64 = 0.2;
const CACHE_PATH: &str = "user://expansive_tile_data.cache";

#[derive(GodotClass, Debug)]
#[class(base=Node2D, init)]
pub struct ImageProcessor {
    #[export]
    pub force_tile_regen: bool,
    #[export]
    pub debug_unmapped: bool,
    #[export]
    pub debug_tile_placement: bool,
    base: Base<Node2D>,
    image: Option<Gd<Image>>,
    tile_ids: Dictionary,
    tile_data: Dictionary,
}



#[godot_api]
impl ImageProcessor {
    #[func]
    fn _ready(&mut self) {
        if !self.base.to_gd().is_instance_valid() {
            log(LogLevel::Error, "🚫 Base Node2D instance is null in _ready");
            return;
        }
        self.base.to_gd().emit_signal("online", &[]);
    }

    #[func]
    fn load_or_generate_tile_data(&mut self) {
        if !self.load_tile_data_cache(CACHE_PATH) || self.force_tile_regen {
            self.prepare_tile_data();
            self.save_tile_data_cache(CACHE_PATH);
        } else {
            log(LogLevel::Info, "💾 Loaded tile data from cache");
        }
        if self.base.to_gd().is_instance_valid() {
            self.base.to_gd().emit_signal("ip_ready", &[]);
        } else {
            log(LogLevel::Error, "🚫 Base Node2D instance is null when emitting ip_ready");
        }
    }

    fn prepare_tile_data(&mut self) {
        let image = self.load_image(IMAGE_PATH);
        if !image.is_instance_valid() {
            log(LogLevel::Error, "🚫 Loaded image is null or invalid");
            return;
        }
        self.image = Some(image.clone());
        self.tile_ids = self.resolve_tile_ids();
        self.convert_image_to_tile_data(image);
    }

    fn load_image(&self, path: &str) -> Gd<Image> {
        ResourceLoader::singleton().load(path).map_or_else(|| {
            log(LogLevel::Error, &format!("🚫 Failed to load texture from path: {}", path));
            Image::create(1, 1, false, Format::RGBA8).unwrap()
        }, |mut tex| {
            tex.call("get_image", &[]).try_to::<Gd<Image>>().unwrap_or_else(|_| {
                log(LogLevel::Error, "🚫 Failed to get image from texture");
                Image::create(1, 1, false, Format::RGBA8).unwrap()
            })
        })
    }

    fn resolve_tile_ids(&self) -> Dictionary {
        let mut ids = Dictionary::new();
        for &(label, coords) in TILE_MAP {
            ids.insert(label, coords);
        }
        ids
    }

    fn convert_image_to_tile_data(&mut self, image: Gd<Image>) {
        if !image.is_instance_valid() {
            log(LogLevel::Error, "🚫 Image instance is null in convert_image_to_tile_data");
            return;
        }
        self.tile_data.clear();
        let mut used_positions = Dictionary::new();
        let mut stats = Dictionary::new();
        stats.insert("unmapped", 0i64);
        stats.insert("alpha", 0i64);
        stats.insert("duplicates", 0i64);

        let (width, height) = (image.get_width(), image.get_height());
        let (half_w, half_h) = (width / 2, height / 2);

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                if pixel.a < 0.5 {
                    stats.insert("alpha", stats.get("alpha").unwrap_or(0i64.to_variant()).to::<i64>() + 1);
                    continue;
                }

                let pos = Vector2i::new(x - half_w, y - half_h);
                let lum = pixel.luminance();
                let label = if lum > THRESHOLD_WHITE {
                    "wall"
                } else if lum < THRESHOLD_BLACK {
                    "floor"
                } else {
                    stats.insert("unmapped", stats.get("unmapped").unwrap_or(0i64.to_variant()).to::<i64>() + 1);
                    if self.debug_unmapped {
                        self.tile_data.insert(pos, tile_payload(0, Vector2i::new(0, 0), 0));
                    }
                    continue;
                };

                let coords = self.tile_ids.get(label).unwrap_or(Variant::nil());
                if coords.is_nil() {
                    continue;
                }

                if used_positions.contains_key(pos) {
                    stats.insert("duplicates", stats.get("duplicates").unwrap_or(0i64.to_variant()).to::<i64>() + 1);
                    continue;
                }

                let atlas_coords = coords.try_to::<Vector2i>().unwrap_or(Vector2i::ZERO);
                self.tile_data.insert(pos, tile_payload(0, atlas_coords, 0));
                used_positions.insert(pos, true);
            }
        }

        if let Some(mut snapshot) = FileAccess::open("res://expansive_scan.bin", ModeFlags::WRITE) {
            snapshot.store_var(&self.tile_data.to_variant());
        } else {
            log(LogLevel::Error, "🚫 Failed to open expansive_scan.bin for writing");
        }

        if self.base.to_gd().is_instance_valid() {

            self.base.to_gd().emit_signal("ip_ready", &[]);
        }
    }

    fn save_tile_data_cache(&self, path: &str) {
        if let Some(mut f) = FileAccess::open(path, ModeFlags::WRITE) {
            f.store_var(&self.tile_data.to_variant());
        } else {
            log(LogLevel::Error, &format!("🚫 Failed to open cache file {} for writing", path));
        }
    }

    fn load_tile_data_cache(&mut self, path: &str) -> bool {
        if !FileAccess::file_exists(path) {
            return false;
        }
        if let Some(f) = FileAccess::open(path, ModeFlags::READ) {
            match f.get_var().try_to::<Dictionary>() {
                Ok(data) => {
                    self.tile_data = data;
                    true
                }
                Err(e) => {
                    log(LogLevel::Error, &format!("🚫 Failed to parse cache data: {}", e));
                    false
                }
            }
        } else {
            log(LogLevel::Error, &format!("🚫 Failed to open cache file {} for reading", path));
            false
        }
    }

    #[func]
    fn get_decoder_payload(&self) -> Dictionary {
        let mut output = Dictionary::new();
        for pos in self.tile_data.keys_array().iter_shared() {
            let pos_vec: Vector2i = pos.try_to().unwrap_or(Vector2i::ZERO);
            let info = self.tile_data.get(pos).unwrap_or_default();
            let info_dict = info.try_to::<Dictionary>().unwrap_or_default();
            let atlas_coords = info_dict.get("atlas_coords").and_then(|v| v.try_to::<Vector2i>().ok()).unwrap_or(Vector2i::ZERO);
            let label = TILE_MAP.iter().find(|&&(_, coord)| coord == atlas_coords).map(|&(l, _)| l).unwrap_or("");
            let mut entry = Dictionary::new();
            entry.insert("x", pos_vec.x);
            entry.insert("y", pos_vec.y);
            entry.insert("source_id", info_dict.get("source_id").unwrap_or(0i64.to_variant()));
            entry.insert("atlas_coords", atlas_coords);
            entry.insert("alternate_id", info_dict.get("alternate_id").unwrap_or(0i64.to_variant()));
            entry.insert("label", label);
            output.insert(pos_vec, entry);
        }
        output
    }
}

fn tile_payload(source_id: i64, atlas_coords: Vector2i, alternate_id: i64) -> Variant {
    let mut dict = Dictionary::new();
    dict.insert("source_id", source_id);
    dict.insert("atlas_coords", atlas_coords);
    dict.insert("alternate_id", alternate_id);
    dict.to_variant()
}