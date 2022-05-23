use super::*;
use crate::prelude::*;

const TOWER_COST: i32 = 150;

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
    gold: Res<gold::Gold>,
) {
    msg_query
        .iter()
        .for_each(|(entity, _, build_tower, target_map)| {
            if let Ok((map_entity, map)) = map_query.get(target_map.0) {
                let tile_entity = map.get_tile_entity_at_coord(build_tower.location);
                if let Ok(tile_type) = tile_query.get(tile_entity) {
                    let tile_valid = *tile_type == TileType::Empty;
                    let sufficient_gold = gold.get() >= TOWER_COST;

                    match (tile_valid, sufficient_gold) {
                        (true, true) => {
                            spawn_tower(
                                map_entity,
                                map,
                                build_tower.location,
                                &mut commands,
                                &models,
                            );
                            gold::send_change_gold_message(&mut commands, -TOWER_COST);
                        }
                        (true, false) => {
                            create_system_alert_message(&mut commands, "Not enough gold.")
                        }
                        _ => create_system_alert_message(&mut commands, "Invalid Location"),
                    }
                }
            }

            commands.entity(entity).insert(IsHandled);
        });
}
