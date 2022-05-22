use crate::prelude::*;

const STARTING_LIVES: i32 = 10;

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(0))
            .add_system(change_lives.run_in_state(GameMode::TDMode))
            .add_system(reset_lives.run_in_state(GameMode::TDMode));
    }
}

pub struct Lives(i32);

impl Lives {
    pub fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Component)]
pub struct ResetLives;

#[derive(Component)]
pub struct ChangeLives(i32);

#[derive(Component)]
pub struct OutOfLives;

pub fn send_reset_lives_message(commands: &mut Commands) {
    commands.spawn().insert(Message).insert(ResetLives);
}

fn reset_lives(query: Query<(Entity, &Message, &ResetLives)>, mut commands: Commands) {
    query.iter().for_each(|(e, _, _)| {
        commands.insert_resource(Lives(STARTING_LIVES));
        commands.entity(e).insert(IsHandled);
    });
}

pub fn send_change_lives_message(commands: &mut Commands, amount: i32) {
    commands.spawn().insert(Message).insert(ChangeLives(amount));
}

fn change_lives(
    mut lives: ResMut<Lives>,
    query: Query<(Entity, &Message, &ChangeLives)>,
    mut commands: Commands,
) {
    query.iter().for_each(|(e, _, change)| {
        lives.0 += change.0;
        eprintln!("Lives are now at {}", lives.0);
        commands.entity(e).insert(IsHandled);

        if lives.0 <= 0 {
            commands.spawn().insert(Message).insert(OutOfLives);
        }
    });
}
