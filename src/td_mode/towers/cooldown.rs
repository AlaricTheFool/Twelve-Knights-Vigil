pub use crate::prelude::*;

const CD_TICK_RATE: u64 = 15;

pub struct CDPlugin;

impl Plugin for CDPlugin {
    fn build(&self, app: &mut App) {
        let mut fixed_stage = SystemStage::parallel();
        fixed_stage.add_system(decrement_cooldowns);

        app.add_stage_before(
            CoreStage::Update,
            "fixed_cd_stages",
            FixedTimestepStage::new(std::time::Duration::from_millis(CD_TICK_RATE))
                .with_stage(fixed_stage),
        );

        app.add_system(handle_cooldown_refresh_messages);
    }
}

#[derive(Component, Copy, Clone)]
pub struct Cooldown {
    max: i32,
    current: i32,
}

impl Cooldown {
    pub fn new(length_in_seconds: f32) -> Self {
        let length_in_millis = (length_in_seconds * 1000.0).floor() as i32;
        Self {
            max: length_in_millis,
            current: length_in_millis,
        }
    }

    pub fn with_new_length(&self, new_length_in_seconds: f32) -> Self {
        let mut new = self.clone();
        let new_length_in_millis = (new_length_in_seconds * 1000.0).floor() as i32;
        new.max = new_length_in_millis;
        new.current = new.current.clamp(0, new.max);

        new
    }

    pub fn progress_milliseconds(&mut self, amount: i32) {
        self.current -= amount;
    }

    pub fn is_ready(&self) -> bool {
        self.current <= 0
    }

    pub fn reset(&mut self) {
        self.current = self.max;
    }

    pub fn refill(&mut self) {
        self.current += self.max;
    }

    pub fn shots_per_second(&self) -> f32 {
        1000.0 / self.max as f32
    }
}

#[derive(Component)]
pub struct ResetCooldown;

fn decrement_cooldowns(mut query: Query<&mut Cooldown>) {
    query.iter_mut().for_each(|mut cd| {
        if !cd.is_ready() {
            cd.progress_milliseconds(CD_TICK_RATE as i32);
        }
    });
}

fn handle_cooldown_refresh_messages(
    message_query: Query<(Entity, &Message, &Target, &ResetCooldown)>,
    mut cooldowns: Query<&mut Cooldown>,
    mut commands: Commands,
) {
    message_query
        .iter()
        .for_each(|(message_entity, _, target, _)| {
            if let Ok(mut cd_to_reset) = cooldowns.get_mut(target.0) {
                cd_to_reset.refill();
            }

            commands.entity(message_entity).insert(IsHandled);
        });
}

pub fn spawn_cd_reset_message(target: Entity, commands: &mut Commands) {
    commands
        .spawn()
        .insert(Message)
        .insert(Target(target))
        .insert(ResetCooldown);
}
