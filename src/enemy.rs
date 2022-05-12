use crate::prelude::*;

pub struct SpawnTimer(pub Timer);

pub struct EnemyModels {
    pub basic: Handle<Scene>,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct TrackFollower {
    track: Entity,
    progress: f32,
    speed: f32,
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
                speed: 1.0,
            })
            .with_children(|p| {
                p.spawn_bundle(TransformBundle {
                    local: Transform::from_xyz(0.0, 1.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)),
                    ..default()
                })
                .with_children(|p| {
                    p.spawn_scene(models.basic.clone());
                });
            });
    }
}

pub fn move_track_followers(
    mut query: Query<&TrackFollower>,
    world: &World,
    mut commands: Commands,
) {
    for t_follower in query.iter() {
        let track = world
            .entity(t_follower.track)
            .get::<Track>()
            .expect("A track follower is following a track that doesn't exist.");
    }
}
