use super::*;
use crate::prelude::*;

const TOWER_COST: i32 = 150;

#[derive(Component)]
pub struct BuildTower {
    pub location: Coordinate,
    pub t_type: TowerType,
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
    tile_query: Query<&TileType>,
    mut commands: Commands,
    mut alerts: ResMut<SystemAlerts>,
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
