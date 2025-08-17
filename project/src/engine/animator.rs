// animator.rs

//! 🎞️ Animator Module
//! This module defines animation logic for tile/voxel fields.
//! It is dimension-agnostic and designed to choreograph motion over time
//! without assuming a specific rendering backend.

use crate::engine::types::{Tile, TileKind};
use crate::engine::dimension::DimensionContext;
use crate::utils::time::{Tick, Duration};

/// Represents an animation applied to a tile
pub struct TileAnimation<P> {
    pub target: P,
    pub kind: AnimationKind,
    pub started_at: Tick,
    pub duration: Duration,
}

/// Enum of supported animation types
#[derive(Clone, Debug)]
pub enum AnimationKind {
    FadeIn,
    FadeOut,
    Pulse,
    Translate { dx: i32, dy: i32, dz: i32 },
}

/// Animator orchestrates tile animations over time
pub struct Animator<P> {
    pub active: Vec<TileAnimation<P>>,
}

impl<P: Clone + PartialEq> Animator<P> {
    pub fn new() -> Self {
        Self { active: Vec::new() }
    }

    /// Schedule a new animation
    pub fn schedule(&mut self, anim: TileAnimation<P>) {
        self.active.push(anim);
    }

    /// Advance animations by one tick
    pub fn tick(&mut self, now: Tick) {
        self.active.retain(|anim| now - anim.started_at < anim.duration);
        // Future: apply effects to tiles or emit signals
    }
}
