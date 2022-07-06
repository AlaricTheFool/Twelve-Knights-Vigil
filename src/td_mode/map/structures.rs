use super::*;

pub struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_to_stage(
            CoreStage::Last,
            place_portals.run_in_state(GameState::TDMode),
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Structure {
    None,
    Barricade,
}

impl Structure {
    pub fn all() -> [Self; 2] {
        [Self::None, Self::Barricade]
    }
}

struct StructureModels {
    wave_entry: Handle<Scene>,
    wave_exit: Handle<Scene>,
    barricade: Handle<Scene>,
}

fn setup(assets: Res<AssetServer>, mut commands: Commands) {
    let structure_models = StructureModels {
        wave_entry: assets.load("models/wave_portal.glb#Scene0"),
        wave_exit: assets.load("models/wave_portal.glb#Scene0"),
        barricade: assets.load("models/barricade.glb#Scene0"),
    };

    commands.insert_resource(structure_models);
}

/// A Tag Component for the entry and exit portals
#[derive(Component)]
struct Portal;

fn place_portals(
    portal_query: Query<Entity, With<Portal>>,
    map_root_query: Query<&MapRoot>,
    map: Res<Map>,
    models: Res<StructureModels>,
    mut commands: Commands,
) {
    if map.is_changed() {
        portal_query.iter().for_each(|e| {
            commands.entity(e).despawn_recursive();
        });

        if !map.is_empty() && !map_root_query.is_empty() {
            let map_root = map_root_query.get_single().unwrap();

            // Spawn entry portal
            let portal_coord = map.wave_entry_coord;
            let portal_idx = map.coord_to_idx(portal_coord);
            let portal_tile = map_root.tile_entities[portal_idx];

            commands.entity(portal_tile).with_children(|p| {
                p.spawn()
                    .insert_bundle(TransformBundle::identity())
                    .insert(Portal)
                    .with_children(|p| {
                        p.spawn_scene(models.wave_entry.clone());
                    });
            });

            // Spawn exit portal
            let portal_coord = map.wave_exit_coord;
            let portal_idx = map.coord_to_idx(portal_coord);
            let portal_tile = map_root.tile_entities[portal_idx];

            commands.entity(portal_tile).with_children(|p| {
                p.spawn()
                    .insert_bundle(TransformBundle::identity())
                    .insert(Portal)
                    .with_children(|p| {
                        p.spawn_scene(models.wave_exit.clone());
                    });
            });
        }
    }
}
