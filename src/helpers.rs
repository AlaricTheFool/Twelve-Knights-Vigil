//! Self-contained utility functions that will no doubt be useful throughout the program.
use std::ops::Mul;

use crate::prelude::*;

/// Convert two pools indicating direction to a float based on which is pressed.
///
/// Used for converting inputs to multipliers.
pub fn bools_to_axis(positive: bool, negative: bool) -> f32 {
    match (positive, negative) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    }
}

/// Convert a rate in seconds to a rate based off the framerate of the fixed timestep
pub fn seconds_rate_to_fixed_rate(val: f32, timestep: u64) -> f32 {
    val * (timestep as f32 / 1000.0)
}

/// Grid Coordinates
#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(item: (usize, usize)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl Mul<Vec3> for Coordinate {
    type Output = Vec3;

    /// Multiplying a coordinate by a vector assumes the coordinate is shifting the vector on the
    /// XZ plane.
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self.x as f32, rhs.y, rhs.z * self.y as f32)
    }
}

/// A pointer to the base of a scene containing models loaded from a gltf format.
#[derive(Component)]
pub struct ModelRoot(pub Entity);
