use crate::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct WeaponPivot(pub Entity);

#[derive(Component)]
pub struct Homing;

#[derive(Copy, Clone, Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Multishot(pub i32);

#[derive(Component, Copy, Clone)]
pub struct Spread(pub f32);

#[derive(Component, Copy, Clone)]
pub struct Damage(pub u32);

#[derive(Component, Copy, Clone)]
pub struct Explosive(pub Range);

#[derive(Component)]
pub struct FireDelay(pub Timer);

#[derive(Component)]
pub struct SpawnPosition(pub Vec3);

#[derive(Component)]
pub struct SpreadPosition(pub Vec3);
