use crate::prelude::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
