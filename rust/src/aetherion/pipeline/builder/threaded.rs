use crate::aetherion::pipeline::data::map_build_options::{MapBuildOptions, GodotNoiseType};
use crate::aetherion::pipeline::data::SerializableVector2i;
use crate::aetherion::pipeline::builder::builder::spawn_map_builder;
use crate::godot4::messaging::GodotSync;

pub fn spawn_builder_thread(sync: GodotSync, options: MapBuildOptions) {
    let config = options.to_noise_config();

    spawn_map_builder(
        sync,
        config,
        GodotNoiseType::from_str(&options.mode.to_string()).to_internal(),

        options.animate,
        options.black.into(),
        options.blue.into(),
    );
}
