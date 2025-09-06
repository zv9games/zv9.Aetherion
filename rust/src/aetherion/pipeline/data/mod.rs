//C:/ZV9/zv9.aetherion/rust/src/aetherion/pipeline/data/mod.rs

pub mod chunk;
pub mod tile;
pub mod vector; // Assuming you'll add vector.rs soon
pub mod map_build_options;
pub mod grid;


pub use chunk::MapDataChunk;
pub use tile::TileInfo;
pub use vector::SerializableVector2i;
pub use map_build_options::MapBuildOptions;
pub use grid::{MapGrid, TileType};

//end mod.rs