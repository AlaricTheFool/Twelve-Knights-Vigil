use crate::prelude::*;

pub struct ProjectilePlugin;

use crate::td_mode::enemy::CenterOfMass;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        let mut fixed_stage = SystemStage::parallel();
        fixed_stage.add_system(move_projectiles);

        app.add_startup_system(initialize_projectile_models)
            .add_system(handle_projectile_spawn_messages)
            .add_stage_before(
                CoreStage::Update,
                "projectile_fixed_stage",
                FixedTimestepStage::new(std::time::Duration::from_millis(16))
                    .with_stage(fixed_stage),
            );
    }
}

struct ProjectileModels {
    ballista_bolt: Handle<Scene>,
}

#[derive(Component)]
struct Speed(f32);

pub enum ProjectileType {
    Ballista,
}

#[derive(Component)]
pub struct SpawnProjectile(ProjectileType);

#[derive(Component)]
pub struct Projectile(ProjectileType);

fn initialize_projectile_models(assets: Res<AssetServer>, mut commands: Commands) {
    let projectile_models = ProjectileModels {
        ballista_bolt: assets.load("models/towers/weapons/ballista_projectile.glb#Scene0"),
    };

    commands.insert_resource(projectile_models);
}

pub fn spawn_projectile_message(
    src: Entity,
    target: Entity,
    projectile_type: ProjectileType,
    commands: &mut Commands,
) {
    commands
        .spawn()
        .insert(Message)
        .insert(Sender(src))
        .insert(Target(target))
        .insert(SpawnProjectile(ProjectileType::Ballista));
}

fn spawn_ballista_bolt(
    start_position: Vec3,
    models: &Res<ProjectileModels>,
    target_entity: Entity,
    commands: &mut Commands,
    parent: Entity,
) {
    commands
        .spawn()
        .insert(Speed(0.1))
        .insert(Target(target_entity))
        .insert(Projectile(ProjectileType::Ballista))
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(start_position),
        ))
        .insert(Parent(parent))
        .with_children(|p| {
            p.spawn_scene(models.ballista_bolt.clone());
        });
}

fn handle_projectile_spawn_messages(
    message_query: Query<(Entity, &Message, &Sender, &Target, &SpawnProjectile)>,
    spawn_point_query: Query<(&Transform, &ProjectileSpawnPoint)>,
    models: Res<ProjectileModels>,
    current_map: Res<CurrentMap>,
    mut commands: Commands,
) {
    message_query
        .iter()
        .for_each(|(entity, _, sender, target, _)| {
            if let Ok((spawn_tform, spawn_point)) = spawn_point_query.get(sender.0) {
                spawn_ballista_bolt(spawn_tform.translation + spawn_point.0, &models, target.0, &mut commands, current_map.0.unwrap());
            } else {
                error!("Attempted to spawn a projectile from a source with no transform or spawn point.")
            }
            commands.entity(entity).insert(IsHandled);
        });
}

fn move_projectiles(
    projectile_query: Query<(Entity, &Transform, &Projectile, &Speed, &Target)>,
    transform_query: Query<(&Transform, &CenterOfMass)>,
    mut commands: Commands,
) {
    projectile_query
        .iter()
        .for_each(|(entity, p_transform, _, speed, target)| {
            if let Ok((target_transform, target_center_of_mass)) = transform_query.get(target.0) {
                let target_pos = target_transform.translation + target_center_of_mass.0;
                let new_transform = p_transform.looking_at(target_pos, Vec3::Y);
                let new_transform = new_transform.with_translation(
                    new_transform.translation + new_transform.forward() * speed.0,
                );

                let remaining_dist = new_transform.translation.distance(target_pos);
                if remaining_dist < 0.25 {
                    commands.entity(entity).despawn_recursive();

                    // TODO: Make this into a message instead.
                    commands.entity(target.0).despawn_recursive();

                    let gold_gained = thread_rng().gen_range(5..15);
                    crate::td_mode::gold::send_change_gold_message(&mut commands, gold_gained);
                } else {
                    // Move towards the projectile's target.
                    commands.entity(entity).insert(new_transform);
                }
            } else {
                commands.entity(entity).despawn_recursive();
            }
        });
}
