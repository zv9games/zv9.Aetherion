// racing.rs

use godot::prelude::*;
use crate::util::{Position, Velocity};

/// 🏁 Racing — Manages racer movement, lap tracking, and race state.
pub struct Racer {
    pub id: GString,
    pub position: Position,
    pub velocity: Velocity,
    pub lap: u32,
    pub finished: bool,
}

impl Racer {
    pub fn new(id: &str, start: Position) -> Self {
        Self {
            id: GString::from(id),
            position: start,
            velocity: Velocity::zero(),
            lap: 0,
            finished: false,
        }
    }

    pub fn update(&mut self, delta: f64, track: &Track) {
        if self.finished {
            return;
        }

        self.position += self.velocity.scale(delta);

        if track.is_lap_complete(self.position) {
            self.lap += 1;
            if self.lap >= track.total_laps {
                self.finished = true;
            }
        }
    }

    pub fn boost(&mut self, amount: f64) {
        self.velocity.increase(amount);
    }
}

/// 🏎️ Track — Defines race boundaries and lap logic.
pub struct Track {
    pub total_laps: u32,
    pub bounds: Vec<Position>,
}

impl Track {
    pub fn is_lap_complete(&self, pos: Position) -> bool {
        // Placeholder logic: lap complete if near start
        self.bounds.first().map_or(false, |start| pos.distance_to(*start) < 1.0)
    }
}
