use crate::prelude::*;

#[derive(Component)]
pub struct WeaponPivot(pub Entity);

#[derive(Component)]
pub struct Homing;

#[derive(Copy, Clone, Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Multishot(pub i32);

#[derive(Component)]
pub struct Spread(pub f32);
