use super::*;
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource,
};

pub struct PickableRaycastSet;

pub struct PickablePlugin;

impl Plugin for PickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<PickableRaycastSet>::default())
            .insert_resource(DefaultPluginState::<PickableRaycastSet>::default())
            .insert_resource(CursorState::NoTarget)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                add_raycast_components_to_tile_meshes.run_in_state(GameState::TDMode),
            )
            .add_system(update_cursor_state.run_in_state(GameState::TDMode))
            .add_system(update_raycast_with_cursor.run_in_state(GameState::TDMode));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorState {
    NoTarget,
    OnTile(Coordinate),
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
    added_mesh_query: Query<Entity, Added<Handle<Mesh>>>,
    tile_query: Query<Entity, Changed<ModelRoot>>,
    children_query: Query<&Children>,
    mesh_query: Query<&Handle<Mesh>>,
    mut commands: Commands,
) {
    added_mesh_query.iter().for_each(|e| {
        info!("Mesh added to entity: {e:?}");
    });
    tile_query.iter().for_each(|entity| {
        info!("Adding raycasting to meshes for entity {entity:?}");
        let children_with_meshes =
            find_children_with_meshes_recursive(entity, &children_query, &mesh_query);

        info!(
            "Adding raycasting to {} meshes.",
            children_with_meshes.len()
        );
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
    tile_query: Query<&Coordinate, With<map::Tile>>,
    mut egui_context: ResMut<bevy_egui::EguiContext>,
    mut cursor_state: ResMut<CursorState>,
) {
    *cursor_state = CursorState::NoTarget;

    if egui_context.ctx_mut().is_pointer_over_area() {
        return;
    }

    let source = source_query.single();

    if let Some((entity, _)) = source.intersect_top() {
        if let Ok(root_entity) = root_query.get(entity) {
            if let Ok(coord) = tile_query.get(root_entity.0) {
                trace!("Tile Hovered: {:?}", coord);
                *cursor_state = CursorState::OnTile(*coord);
            }
        }
    }
}
