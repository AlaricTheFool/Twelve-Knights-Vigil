use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
pub struct Heal(pub u32);

#[derive(Component)]
pub struct Harm(pub u32);

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn get_current(&self) -> u32 {
        self.current
    }

    pub fn get_max(&self) -> u32 {
        self.max
    }

    pub fn healed(&self, amount: u32) -> Self {
        Self {
            current: (self.current + amount).min(self.max),
            max: self.max,
        }
    }

    pub fn harmed(&self, amount: u32) -> Self {
        Self {
            current: self.current.saturating_sub(amount),
            max: self.max,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

pub fn handle_heal_messages(
    health_query: Query<&Health>,
    message_query: Query<(Entity, &Message, &Target, &Heal)>,
    mut commands: Commands,
) {
    message_query
        .iter()
        .for_each(|(message_e, _, target, heal)| {
            if let Ok(health) = health_query.get(target.0) {
                commands.entity(target.0).insert(health.healed(heal.0));
            }

            commands.entity(message_e).insert(IsHandled);
        });
}

pub fn handle_harm_messages(
    health_query: Query<&Health>,
    message_query: Query<(Entity, &Message, &Target, &Harm)>,
    mut commands: Commands,
) {
    message_query
        .iter()
        .for_each(|(message_e, _, target, harm)| {
            if let Ok(health) = health_query.get(target.0) {
                commands.entity(target.0).insert(health.harmed(harm.0));
            }

            commands.entity(message_e).insert(IsHandled);
        });
}

pub fn kill_dead_enemies(enemy_query: Query<(Entity, &Health, &Enemy)>, mut commands: Commands) {
    enemy_query.iter().for_each(|(entity, health, _)| {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
        }
    });
}
