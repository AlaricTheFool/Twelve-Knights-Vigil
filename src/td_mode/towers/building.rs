use super::*;
use crate::prelude::*;

const TOWER_COST: i32 = 150;

#[derive(Component)]
pub struct BuildTower {
    pub location: Coordinate,
    pub t_type: TowerType,
}

#[derive(Component)]
pub struct PlaceKnight {
    pub location: Coordinate,
    pub knight: Knight,
}

pub fn draw_tower_build_ui(mut egui_context: ResMut<EguiContext>, mut ui_action: ResMut<UIAction>) {
    egui::TopBottomPanel::bottom("Tower Building").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            [TowerType::Short, TowerType::Medium, TowerType::Tall]
                .iter()
                .for_each(|t_type| {
                    let txt = match *t_type {
                        TowerType::Short => "Short Tower",
                        TowerType::Medium => "Medium Tower",
                        TowerType::Tall => "Tall Tower",
                    };

                    if ui.button(txt).is_pointer_button_down_on() {
                        *ui_action = UIAction::BuildTower(t_type.clone());
                    }
                });
        });
    });
}

pub fn handle_build_tower_messages(
    msg_query: Query<(Entity, &Message, &BuildTower, &Target)>,
    map_query: Query<(Entity, &TileMap)>,
    tower_query: Query<(&Tower, &Coordinate)>,
    tile_query: Query<&TileType>,
    mut commands: Commands,
    mut alerts: ResMut<SystemAlerts>,
    models: Res<TowerModels>,
    gold: Res<gold::Gold>,
) {
    msg_query
        .iter()
        .for_each(|(entity, _, build_tower, target_map)| {
            // TODO: Use a Result enum to make all these checks and return a single error message
            // on failure.
            if let Ok((map_entity, map)) = map_query.get(target_map.0) {
                let tile_entity = map.get_tile_entity_at_coord(build_tower.location).unwrap();
                if let Ok(tile_type) = tile_query.get(tile_entity) {
                    let tower_on_tile = tower_query
                        .iter()
                        .filter(|(_, coord)| **coord == build_tower.location)
                        .next()
                        .is_none();
                    let tile_valid = *tile_type == TileType::Empty && tower_on_tile;
                    let sufficient_gold = gold.get() >= TOWER_COST;

                    match (tile_valid, sufficient_gold) {
                        (true, true) => {
                            spawn_tower(
                                map_entity,
                                map,
                                build_tower.location,
                                &mut commands,
                                &models,
                                build_tower.t_type,
                            );
                            gold::send_change_gold_message(&mut commands, -TOWER_COST);
                        }
                        (true, false) => {
                            create_system_alert_message(&mut alerts, "Not enough gold.")
                        }
                        _ => create_system_alert_message(&mut alerts, "Invalid Location"),
                    }
                }
            }

            commands.entity(entity).insert(IsHandled);
        });
}

pub fn handle_place_knight_messages(
    msg_query: Query<(Entity, &Message, &PlaceKnight, &Target)>,
    tower_query: Query<(Entity, &Tower, &Coordinate, &TowerType)>,
    occupied_tower_query: Query<(Entity, &Tower, &Knight)>,
    mut commands: Commands,
    mut alerts: ResMut<SystemAlerts>,
    mut knight_statuses: ResMut<KnightStatuses>,
) {
    msg_query.iter().for_each(|(entity, _, place_knight, _)| {
        let mut result = Ok(1);
        let tower_on_tile = tower_query
            .iter()
            .filter(|(_, _, coord, _)| **coord == place_knight.location)
            .map(|(e, _, _, t)| (e, t))
            .next();

        if tower_on_tile.is_none() {
            result = Err("You must place a knight on top of an existing Tower.");
        } else if knight_statuses.get_status(place_knight.knight) != KUsageStatus::Ready {
            result = Err("You can't use that knight!");
        } else if occupied_tower_query.get(tower_on_tile.unwrap().0).is_ok() {
            result = Err("There's already a knight there.");
        }

        match result {
            Ok(_) => {
                knight_statuses.set_status(place_knight.knight, KUsageStatus::InUse);
                add_knight_to_tower(
                    tower_on_tile.unwrap().0,
                    *tower_on_tile.unwrap().1,
                    place_knight.knight,
                    &mut commands,
                );
            }

            Err(msg) => {
                create_system_alert_message(&mut alerts, msg);
            }
        }

        commands.entity(entity).insert(IsHandled);
    });
}
