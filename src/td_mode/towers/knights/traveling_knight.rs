use crate::prelude::*;

const FIXED_TIMESTEP_MILLIS: u64 = 17;

pub struct TravelingKnightPlugin;

impl Plugin for TravelingKnightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_traveling_knight_models)
            .add_system(add_models_to_traveling_knights)
            .add_system(parent_traveling_knights_to_map);

        let mut fixed_stage = SystemStage::parallel();
        fixed_stage.add_system(move_traveling_knights);
        let mut fixed_post_stage = SystemStage::parallel();
        fixed_post_stage.add_system(traveling_knights_arrive);
        app.add_stage_before(
            CoreStage::Update,
            "traveling fixed stage",
            FixedTimestepStage::new(std::time::Duration::from_millis(FIXED_TIMESTEP_MILLIS))
                .with_stage(fixed_stage)
                .with_stage(fixed_post_stage),
        );
    }
}

#[derive(Component)]
pub struct Travel;

#[derive(Component)]
pub struct Arrived;

struct TravelingKnightModels {
    placeholder: Handle<Mesh>,
    placeholder_mat: Handle<StandardMaterial>,
}

pub fn spawn_traveling_knight(
    commands: &mut Commands,
    knight_type: Knight,
    start_point: Transform,
    target_building: Entity,
) {
    commands
        .spawn()
        .insert(Travel)
        .insert_bundle(TransformBundle::from_transform(start_point))
        .insert(Target(target_building))
        .insert(knight_type);
}

fn initialize_traveling_knight_models(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let models = TravelingKnightModels {
        placeholder: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
        placeholder_mat: materials.add(Color::rgb(0.4, 0.4, 0.2).into()),
    };

    commands.insert_resource(models);
}

fn add_models_to_traveling_knights(
    k_query: Query<(Entity, &Knight, &Transform, &GlobalTransform), Added<Travel>>,
    models: Res<TravelingKnightModels>,
    mut commands: Commands,
) {
    k_query.iter().for_each(|(e, k, tform, g_tform)| {
        commands.entity(e).insert_bundle(PbrBundle {
            mesh: models.placeholder.clone(),
            material: models.placeholder_mat.clone(),
            transform: *tform,
            global_transform: *g_tform,
            ..default()
        });
    });
}

fn parent_traveling_knights_to_map(
    k_query: Query<Entity, (Added<Travel>, With<Knight>)>,
    mut commands: Commands,
    map: Res<CurrentMap>,
) {
    k_query.iter().for_each(|e| {
        let m_entity = map
            .0
            .expect("Attempted to spawn a traveling knight without a map to parent it to.");
        commands.entity(e).insert(Parent(m_entity));
    });
}

fn move_traveling_knights(
    k_query: Query<(Entity, &Target, &Transform), With<Travel>>,
    tform_query: Query<&Transform>,
    mut commands: Commands,
) {
    k_query.iter().for_each(|(e, target, tform)| {
        if let Ok(goal_tform) = tform_query.get(target.0) {
            const KNIGHT_SPEED: f32 = 0.1;
            let new_tform = tform.looking_at(goal_tform.translation, Vec3::Y);
            let new_tform =
                new_tform.with_translation(tform.translation + new_tform.forward() * KNIGHT_SPEED);
            commands.entity(e).insert(new_tform);

            if new_tform.translation.distance(goal_tform.translation) <= KNIGHT_SPEED {
                commands.entity(e).insert(Arrived);
            }
        }
    });
}

fn traveling_knights_arrive(
    arrived_query: Query<(Entity, &Target, &Knight), (With<Travel>, Added<Arrived>)>,
    mut kstatuses: ResMut<KnightStatuses>,
    mut commands: Commands,
) {
    arrived_query.iter().for_each(|(e, target, knight)| {
        kstatuses.set_status(*knight, KUsageStatus::Ready);
        commands.entity(target.0).insert(*knight);
        commands.entity(e).despawn_recursive();
    });
}
