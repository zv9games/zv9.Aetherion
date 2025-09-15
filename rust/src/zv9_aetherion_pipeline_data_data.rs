//C:/ZV9/zv9.aetherion/rust/src/zv9_aetherion_pipeline_data_data.rs

// ✅ Suggestions for aetherion/pipeline/data/data.rs

// 🔧 Consider adding grouped re-exports for clarity:
//     - e.g. `pub mod prelude` to expose commonly used types
//     - Useful for simplifying imports in downstream modules

// 🧩 Add documentation for each re-export:
//     - Inline comments or doc aliases to clarify usage
//     - e.g. `/// Tile metadata used in terrain generation`

// 🚦 Add conditional re-exports if needed:
//     - e.g. `#[cfg(feature = "editor")] pub use super::tile::EditorTileInfo;`
//     - Enables modular builds or feature gating

// 📚 Document intended usage pattern:
//     - Clarify that this module is meant for ergonomic access to core types
//     - Could include example usage in doc comment

// 🧪 Add test module if logic expands:
//     - Currently no logic, but if helper functions are added, include `#[cfg(test)]`

// 🧼 Optional: Add type aliases for common combinations:
//     - e.g. `pub type TileMap = HashMap<SerializableVector2i, TileInfo>;`
//     - Improves readability and reuse

// 🚀 Future: Expand to include additional core types:
//     - e.g. `MapBuildOptions`, `NoiseConfig`, or `PatternType`
//     - Useful for centralizing procedural data access

// 🧠 Consider exposing this module as part of public API surface:
//     - e.g. `pub use crate::aetherion::pipeline::data::*;` in lib.rs
//     - Makes it easier for external crates or Godot bindings to consume


// Re-exports core data types used in procedural generation.

pub use crate::zv9_prelude::*;



// the end