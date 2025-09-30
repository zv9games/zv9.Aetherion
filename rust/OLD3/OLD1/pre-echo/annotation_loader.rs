use godot::prelude::*;
use godot::builtin::{Dictionary, VariantArray, GString};
use godot::classes::{Image, TileMap};
use crate::changeover::ChangeOver;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::logging::{log, LogLevel};


#[derive(GodotClass, Debug)]
#[class(base=Node2D, init)]
pub struct AnnotationLoader {
    #[export]
    debug_annotations: bool,
    #[export]
    save_path: GString,
    base: Base<Node2D>,
    tile_ids: Dictionary,
    image: Option<Gd<Image>>,
    tilemap_layer: Option<Gd<TileMap>>,
    annotation_queue: Vec<(i32, i32, String)>,
    yield_rate: usize,
}

#[godot_api]
impl AnnotationLoader {
    #[func]
    fn _ready(&mut self) {
        log(LogLevel::Info, "🚀 AnnotationLoader initializing");
        if !self.base.to_gd().is_instance_valid() {
            log(LogLevel::Error, "🚫 Base Node2D instance is null in _ready");
            return;
        }
        log(LogLevel::Info, "✅ Base Node2D instance is valid");
        if self.image.is_some() && !self.image.as_ref().unwrap().is_instance_valid() {
            log(LogLevel::Warn, "⚠️ Image instance is null or invalid");
            self.image = None;
        }
        if self.tilemap_layer.is_some() && !self.tilemap_layer.as_ref().unwrap().is_instance_valid() {
            log(LogLevel::Warn, "⚠️ TileMap instance is null or invalid");
            self.tilemap_layer = None;
        }
        log(LogLevel::Info, &format!("📁 Save path: {}", self.save_path));
        self.base.to_gd().emit_signal("online", &[]);
        log(LogLevel::Info, "📡 Emitted online signal");
    }

    #[func]
    fn resolve_tile_ids(&mut self) -> Dictionary {
        log(LogLevel::Info, "🗺️ Resolving tile IDs");
        let dict = Dictionary::new();
        log(LogLevel::Info, "✅ Created empty tile IDs dictionary");
        dict
    }

    #[func]
    fn preload_annotations(&mut self) {
        log(LogLevel::Info, &format!("📂 Preloading annotations from {}", self.save_path));
        self.annotation_queue.clear();
        log(LogLevel::Info, "🧹 Cleared annotation queue");

        match File::open(&self.save_path.to_string()) {
            Ok(file) => {
                log(LogLevel::Info, "✅ Opened annotation file successfully");
                let reader = BufReader::new(file);
                let mut line_count = 0;

                for line in reader.lines().flatten() {
                    line_count += 1;
                    log(LogLevel::Info, &format!("📜 Processing line #{}", line_count));
                    let parts: Vec<&str> = line.trim().split(',').collect();
                    if parts.len() != 3 {
                        log(LogLevel::Warn, &format!("⚠️ Invalid line format at line #{}: {:?}", line_count, line));
                        continue;
                    }

                    let x = parts[0].parse::<i32>().unwrap_or_else(|e| {
                        log(LogLevel::Warn, &format!("⚠️ Failed to parse x at line #{}: {}", line_count, e));
                        0
                    });
                    let y = parts[1].parse::<i32>().unwrap_or_else(|e| {
                        log(LogLevel::Warn, &format!("⚠️ Failed to parse y at line #{}: {}", line_count, e));
                        0
                    });
                    let label = parts[2].to_string();
                    log(LogLevel::Info, &format!("🧱 Annotation: pos=({},{}), label={}", x, y, label));

                    self.annotation_queue.push((x, y, label));
                }
                log(LogLevel::Info, &format!("✅ Loaded {} annotations", self.annotation_queue.len()));
            }
            Err(e) => {
                log(LogLevel::Error, &format!("🚫 Failed to open annotation file {}: {}", self.save_path, e));
            }
        }
    }

    #[func]
    fn apply_annotation_payload(&mut self, payload: VariantArray) {
        log(LogLevel::Info, &format!("🔮 Starting apply_annotation_payload with {} entries", payload.len()));
        if !self.base.to_gd().is_instance_valid() {
            log(LogLevel::Error, "🚫 Base Node2D instance is null in apply_annotation_payload");
            return;
        }
        log(LogLevel::Info, "✅ Base Node2D instance is valid");

        if self.tilemap_layer.is_some() && !self.tilemap_layer.as_ref().unwrap().is_instance_valid() {
            log(LogLevel::Warn, "⚠️ TileMap instance is null or invalid, clearing reference");
            self.tilemap_layer = None;
        }

        let mut total = 0;
        let mut batch = 0;

        for (i, entry) in payload.iter_shared().enumerate() {
            log(LogLevel::Info, &format!("🔍 Processing entry #{}", i));
            let arr = match entry.try_to::<VariantArray>() {
                Ok(arr) => arr,
                Err(e) => {
                    log(LogLevel::Warn, &format!("⚠️ Entry #{} is not an array, error: {} — skipping", i, e));
                    continue;
                }
            };
            log(LogLevel::Info, &format!("📋 Entry #{} is a valid array with {} elements", i, arr.len()));

            if arr.len() < 3 {
                log(LogLevel::Warn, &format!("⚠️ Malformed entry #{} with {} elements — skipping", i, arr.len()));
                continue;
            }

            let x = arr.get(0).and_then(|v| v.try_to::<i32>().ok()).unwrap_or_else(|| {
                log(LogLevel::Warn, &format!("⚠️ Invalid x coordinate in entry #{}", i));
                0
            });
            let y = arr.get(1).and_then(|v| v.try_to::<i32>().ok()).unwrap_or_else(|| {
                log(LogLevel::Warn, &format!("⚠️ Invalid y coordinate in entry #{}", i));
                0
            });
            let label = arr.get(2).and_then(|v| v.try_to::<String>().ok()).unwrap_or_else(|| {
                log(LogLevel::Warn, &format!("⚠️ Invalid label in entry #{}", i));
                String::default()
            });

            log(LogLevel::Info, &format!("🧱 Annotation #{}: pos=({},{}), label={}", i, x, y, label));

            total += 1;
            batch += 1;

            if batch >= self.yield_rate {
                log(LogLevel::Info, &format!("⏸️ Yield point reached after processing {} entries", batch));
                batch = 0;
            }
        }

        log(LogLevel::Info, &format!("🔮 Rust annotation payload applied — {} glyphs placed", total));
        if self.base.to_gd().is_instance_valid() {
            self.base.to_gd().emit_signal("al_ready", &[]);
            log(LogLevel::Info, "📡 Emitted al_ready signal");
        } else {
            log(LogLevel::Error, "🚫 Base Node2D instance is null when emitting al_ready");
        }
    }
}