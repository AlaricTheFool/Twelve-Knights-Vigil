use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
