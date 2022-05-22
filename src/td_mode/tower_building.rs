use crate::prelude::*;

#[derive(Component)]
pub struct BuildTower {
    pub location: Coordinate,
}

pub fn handle_build_tower_messages(
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
