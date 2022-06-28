use super::*;

#[derive(Component)]
pub struct Message;

/// Tag component indicating a message has been handled and can be deleted.
#[derive(Component)]
pub struct Handled;

/// Tag component indicating a messages target.
#[derive(Component)]
pub struct Target(pub Entity);

/// Tag component indicating a messages sender.
#[derive(Component)]
pub struct Source(pub Entity);

pub fn clear_handled_messages(
    m_query: Query<Entity, (With<Message>, With<Handled>)>,
    mut commands: Commands,
) {
    m_query.iter().for_each(|e| {
        commands.entity(e).despawn();
    });
}
