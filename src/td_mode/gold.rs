use crate::prelude::*;

const STARTING_GOLD: i32 = 200;

pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gold(0))
            .add_system(change_gold.run_in_state(GameMode::TDMode))
            .add_system(reset_gold.run_in_state(GameMode::TDMode));
    }
}

pub struct Gold(i32);

impl Gold {
    pub fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Component)]
pub struct ResetGold;

#[derive(Component)]
pub struct ChangeGold(i32);

pub fn send_reset_gold_message(commands: &mut Commands) {
    commands.spawn().insert(Message).insert(ResetGold);
}

fn reset_gold(
    query: Query<(Entity, &Message, &ResetGold)>,
    mut commands: Commands,
    cheats: Res<Cheats>,
) {
    query.iter().for_each(|(e, _, _)| {
        if cheats.infinite_money {
            commands.insert_resource(Gold(999999));
        } else {
            commands.insert_resource(Gold(STARTING_GOLD));
        }
        commands.entity(e).insert(IsHandled);
    });
}

pub fn send_change_gold_message(commands: &mut Commands, amount: i32) {
    commands.spawn().insert(Message).insert(ChangeGold(amount));
}

fn change_gold(
    mut gold: ResMut<Gold>,
    query: Query<(Entity, &Message, &ChangeGold)>,
    mut commands: Commands,
    cheats: Res<Cheats>,
) {
    query.iter().for_each(|(e, _, change)| {
        if !cheats.infinite_money {
            gold.0 += change.0;
            gold.0 = gold.0.max(0);
        }
        commands.entity(e).insert(IsHandled);
    });
}
