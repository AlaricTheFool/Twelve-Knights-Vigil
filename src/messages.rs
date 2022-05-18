use crate::prelude::*;

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<BuildTower>()
                .register_inspectable::<Message>();
        }

        app.add_system_to_stage(CoreStage::Last, clear_handled_messages);
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct Message {
    pub is_handled: bool,
}

impl Message {
    pub fn new() -> Self {
        Self { is_handled: false }
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct BuildTower {
    pub location: Coordinate,
}

fn clear_handled_messages(query: Query<(Entity, &Message)>, mut commands: Commands) {
    query.iter().for_each(|(entity, msg)| {
        if msg.is_handled {
            commands.entity(entity).despawn_recursive();
        }
    });
}
