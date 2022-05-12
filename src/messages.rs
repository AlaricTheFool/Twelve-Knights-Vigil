use crate::prelude::*;

#[derive(Component)]
pub struct Message;

#[derive(Component)]
pub struct Target {
    entity: Entity,
}

#[derive(Component)]
pub struct NewTransform {
    transform: Transform,
}
