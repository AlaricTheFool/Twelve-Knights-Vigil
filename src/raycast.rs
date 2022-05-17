use crate::prelude::*;
use bevy_mod_raycast::{DefaultPluginState, DefaultRaycastingPlugin, Intersection, RayCastMethod};

pub struct PickableRaycastSet;

pub struct PickablePlugin;

impl Plugin for PickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<PickableRaycastSet>::default())
            .insert_resource(DefaultPluginState::<PickableRaycastSet>::default())
            .add_system(add_raycast_components_to_tile_meshes)
            .add_system(update_cursor_state)
            .add_system(update_raycast_with_cursor);

        #[cfg(feature = "debug")]
        app.insert_resource(
            DefaultPluginState::<PickableRaycastSet>::default().with_debug_cursor(),
        );
    }
}

#[derive(Component)]
struct RootEntity(Entity);

pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<PickableRaycastSet>>,
) {
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query.iter_mut() {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}

fn add_raycast_components_to_tile_meshes(
    tile_query: Query<(Entity, &Tile), Added<Tile>>,
    children_query: Query<&Children>,
    mesh_query: Query<&Handle<Mesh>>,
    mut commands: Commands,
) {
    tile_query.iter().for_each(|(entity, _)| {
        let children_with_meshes =
            find_children_with_meshes_recursive(entity, &children_query, &mesh_query);

        children_with_meshes.iter().for_each(|e| {
            commands
                .entity(*e)
                .insert(RayCastMesh::<PickableRaycastSet>::default())
                .insert(RootEntity(entity));
        })
    });
}

fn find_children_with_meshes_recursive(
    entity: Entity,
    children_query: &Query<&Children>,
    mesh_query: &Query<&Handle<Mesh>>,
) -> Vec<Entity> {
    let mut result = Vec::new();

    if let Ok(children) = children_query.get(entity) {
        children.iter().for_each(|child_entity| {
            result.append(&mut find_children_with_meshes_recursive(
                *child_entity,
                children_query,
                mesh_query,
            ));
        });
    }

    if let Ok(_mesh) = mesh_query.get(entity) {
        result.push(entity);
    }

    return result;
}

fn update_cursor_state(
    source_query: Query<&RayCastSource<PickableRaycastSet>>,
    root_query: Query<&RootEntity>,
    tile_query: Query<&Tile>,
) {
    let source = source_query.single();

    if let Some((entity, _)) = source.intersect_top() {
        let root_entity = root_query
            .get(entity)
            .expect("There is a pickable tile with no root entity!");
        let tile = tile_query
            .get(root_entity.0)
            .expect("There is a tile root entity without a tile component.");

        eprintln!("Tile Hovered: [{}, {}]", tile.x, tile.y);
    }
}
