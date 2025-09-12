//C:/ZV9/zv9.aetherion/rust/src/shared/traits.rs

/// ✅ Suggestions for shared/traits.rs

// 🔧 Add default implementations where possible:
//     - e.g. `impl<T: Serialize + DeserializeOwned> Serializable for T` via serde
//     - Reduces boilerplate and improves ergonomics

// 🧩 Add async variant of `Tickable` if needed:
//     - `async fn tick_async(&mut self, delta_time: f32)`
//     - Useful for systems that rely on async IO or deferred updates

// 🚦 Add error handling to `Serializable::deserialize()`:
//     - Return `Result<Self, String>` or use `anyhow::Result`
//     - Prevents panics and improves robustness

// 📚 Document intended use cases:
//     - Clarify which systems implement `Tickable` (e.g. engine, UI, particles)
//     - Note that `Serializable` is for string-based formats, not binary

// 🧪 Add trait tests or examples:
//     - Validate tick behavior in mock systems
//     - Ensure serialization round-trips correctly

// 🧼 Optional: Add `Resettable` or `Initializable` trait:
//     - e.g. `fn reset(&mut self)` or `fn init_from_config(...)`
//     - Useful for lifecycle management in procedural systems

// 🚀 Future: Add trait composition or tagging:
//     - e.g. `trait Procedural: Tickable + Serializable`
//     - Enables unified interfaces for runtime entities

// 🧠 Consider integrating with Godot's lifecycle:
//     - Wrap `Tickable` into a `GodotTickable` trait for `_process()` bridging
//     - Improves interop and editor integration


//! Shared traits used across Aetherion modules.
//! These define core behaviors like ticking and serialization.

/// Trait for objects that can be updated each tick.
/// Used in runtime systems, animations, and procedural logic.
pub trait Tickable {
    /// Called once per frame with the elapsed time in seconds.
    fn tick(&mut self, delta_time: f32);
}

/// Trait for serializable game data.
/// Allows conversion to and from string-based formats.
pub trait Serializable {
    /// Serializes the object into a string.
    fn serialize(&self) -> String;

    /// Deserializes the object from a string.
    fn deserialize(s: &str) -> Self
    where
        Self: Sized;
}
//end traits.rs