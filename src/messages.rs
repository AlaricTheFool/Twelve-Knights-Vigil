use crate::prelude::*;

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<BuildTower>();
        }

        app.add_system(handle_build_tower_messages)
            .add_system_to_stage(CoreStage::Last, clear_handled_messages);
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

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct BuildTower {
    pub location: Coordinate,
}

fn handle_build_tower_messages(
    msg_query: Query<(Entity, &Message, &BuildTower, &Target)>,
    map_query: Query<(Entity, &TileMap)>,
    tile_query: Query<&TileType>,
    mut commands: Commands,
    models: Res<TowerModels>,
) {
    msg_query
        .iter()
        .for_each(|(entity, _, build_tower, target_map)| {
            if let Ok((map_entity, map)) = map_query.get(target_map.0) {
                let tile_entity = map.get_tile_entity_at_coord(build_tower.location);
                if let Ok(tile_type) = tile_query.get(tile_entity) {
                    if *tile_type == TileType::Empty {
                        spawn_tower(
                            map_entity,
                            map,
                            build_tower.location,
                            &mut commands,
                            &models,
                        );
                    }
                }
            }

            commands.entity(entity).insert(IsHandled);
        });
}

fn clear_handled_messages(query: Query<(Entity, &Message, &IsHandled)>, mut commands: Commands) {
    query.iter().for_each(|(entity, _, _)| {
        commands.entity(entity).despawn_recursive();
    });
}
