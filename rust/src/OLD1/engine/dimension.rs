// dimension.rs

//! 🤖 Bot Flipper — Dimensional Keystone
//! This module defines enforced dual-mode spatial logic for EchoEngine.
//! It supports runtime flipping between 2D and 3D contexts.
//! All downstream modules must respect the active dimensional mode.

use std::fmt::Debug;
use std::hash::Hash;

/// Canonical position types
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dim2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dim3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Unified position enum for runtime flipping
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Position {
    TwoD(Dim2),
    ThreeD(Dim3),
}

/// Trait for dimension-aware modules
pub trait DimensionContext {
    fn position(x: i32, y: i32, z: Option<i32>) -> Position;
    fn is_3d(&self) -> bool;
    fn flip(&mut self);
}

/// Bot Flipper — runtime dimensional mode
#[derive(Clone, Debug)]
pub struct BotFlipper {
    pub use_3d: bool,
}

impl BotFlipper {
    pub fn new(use_3d: bool) -> Self {
        Self { use_3d }
    }
}

impl DimensionContext for BotFlipper {
    fn position(x: i32, y: i32, z: Option<i32>) -> Position {
        match z {
            Some(z_val) => Position::ThreeD(Dim3 { x, y, z: z_val }),
            None => Position::TwoD(Dim2 { x, y }),
        }
    }

    fn is_3d(&self) -> bool {
        self.use_3d
    }

    fn flip(&mut self) {
        self.use_3d = !self.use_3d;
    }
}
