use crate::prelude::*;

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Last, clear_handled_messages);
    }
}

#[derive(Component)]
pub struct Message;

#[derive(Component)]
pub struct IsHandled;

#[derive(Component)]
pub struct Target(pub Entity);

#[derive(Component)]
pub struct Sender(pub Entity);

fn clear_handled_messages(query: Query<(Entity, &Message, &IsHandled)>, mut commands: Commands) {
    query.iter().for_each(|(entity, _, _)| {
        commands.entity(entity).despawn_recursive();
    });
}
