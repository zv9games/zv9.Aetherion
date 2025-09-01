// shared/grid_bounds.rs
use crate::shared::vector::SerializableVector2i;

#[derive(Debug, Clone, Copy)]
pub struct GridBounds {
    pub width: i32,
    pub height: i32,
    pub origin: SerializableVector2i,
}

impl GridBounds {
    pub fn contains(&self, pos: SerializableVector2i) -> bool {
        let rel_x = pos.x - self.origin.x;
        let rel_y = pos.y - self.origin.y;
        rel_x >= 0 && rel_y >= 0 && rel_x < self.width && rel_y < self.height
    }
}
