use aetherion_shared::AetherionData;
use glam::Vec3;

// Example function that uses a core dependency (glam)
pub fn calculate_vector_magnitude(v: Vec3) -> f32 {
    v.length()
}

pub fn process_data(data: &AetherionData) -> f64 {
    data.value * 2.0
}