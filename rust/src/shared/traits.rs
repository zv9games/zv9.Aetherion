//! Shared traits used across modules

/// Trait for objects that can be updated each tick
pub trait Tickable {
    fn tick(&mut self, delta_time: f32);
}

/// Trait for serializable game data
pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(s: &str) -> Self where Self: Sized;
}
