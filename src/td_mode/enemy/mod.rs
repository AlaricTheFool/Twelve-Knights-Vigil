use super::*;

mod health;

pub use health::*;

pub struct SpawnTimer(pub Timer);

pub struct EnemyModels {
    pub basic: Handle<Scene>,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct TrackFollower {
    track: Entity,
    pub progress: f32,
    speed: f32,
}

#[derive(Component)]
pub struct CenterOfMass(pub Vec3);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_heal_messages)
            .add_system(handle_harm_messages)
            .add_system_to_stage(CoreStage::PostUpdate, kill_dead_enemies);
    }
}

pub fn spawn_enemies(
    models: Res<EnemyModels>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    map: Res<CurrentMap>,
    mut commands: Commands,
) {
    if map.0.is_some() && timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn_bundle(TransformBundle::from(Transform { ..default() }))
            .insert(Name::new(format!("Enemy")))
            .insert(Enemy)
            .insert(TrackFollower {
                track: map.0.unwrap(),
                progress: 0.0,
                speed: 0.05,
            })
            .insert(Parent(map.0.unwrap()))
            .insert(CenterOfMass(Vec3::Y * 0.5))
            .insert(Health::new(10))
            .with_children(|p| {
                p.spawn_bundle(TransformBundle {
                    local: Transform::from_xyz(0.0, 0.4, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn_scene(models.basic.clone());
                });
            });
    }
}

pub fn move_track_followers(mut query: Query<&mut TrackFollower>) {
    for mut t_follower in query.iter_mut() {
        t_follower.progress += t_follower.speed;
    }
}

pub fn update_track_followers(
    query: Query<(Entity, &TrackFollower)>,
    world: &World,
    mut commands: Commands,
) {
    for (entity, t_follower) in query.iter() {
        let track = world
            .entity(t_follower.track)
            .get::<Track>()
            .expect("A track follower is following a track that doesn't exist.");

        if t_follower.progress < track.length {
            let new_transform = track.get_point(t_follower.progress);

            commands.entity(entity).insert(new_transform);
        } else {
            life::send_change_lives_message(&mut commands, -1);
            commands.entity(entity).despawn_recursive();
        }
    }
}
