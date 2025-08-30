use super::vector::SerializableVector2i;

/// Configuration options for procedural map generation.
#[derive(Debug, Clone)]
pub struct MapBuildOptions {
    pub width: i32,
    pub height: i32,
    pub seed: u64,
    pub mode: String,
    pub animate: bool,
    pub black: SerializableVector2i,
    pub blue: SerializableVector2i,
}
