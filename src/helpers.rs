//! Self-contained utility functions that will no doubt be useful throughout the program.

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
