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

#[derive(Component)]
pub struct PowerSlider(f32);

impl PowerSlider {
    pub fn new() -> Self {
        Self(0.5)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn with_value(val: f32) -> Self {
        Self(val.clamp(0.0, 1.0))
    }

    pub fn get_reverse(&self) -> f32 {
        1.0 - self.0
    }
}

#[derive(Component, Copy, Clone)]
pub struct Damage(pub u32);
