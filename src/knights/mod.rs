use crate::prelude::*;
use std::collections::BTreeMap;

#[derive(Copy, Clone, PartialEq)]
pub enum KUsageStatus {
    Ready,
    InUse,
    Locked,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Clone, Component)]
pub enum Knight {
    Normal,
    Swole,
    Lizard,
    Dungeon,
    Samurai,
    Banner,
}

impl Knight {
    pub fn get_name(&self) -> &str {
        match *self {
            Knight::Normal => "Normal Knight",
            Knight::Swole => "Swole Knight",
            Knight::Lizard => "Lizard Knight",
            Knight::Banner => "Banner Knight",
            Knight::Dungeon => "Dungeon Knight",
            _ => "The Knight with No Name",
        }
    }
}

pub struct KnightStatuses(pub BTreeMap<Knight, KUsageStatus>);

impl KnightStatuses {
    fn new() -> Self {
        Self(BTreeMap::from([
            (Knight::Normal, KUsageStatus::Ready),
            (Knight::Swole, KUsageStatus::Locked),
            (Knight::Lizard, KUsageStatus::Locked),
            (Knight::Banner, KUsageStatus::Locked),
            (Knight::Dungeon, KUsageStatus::Locked),
        ]))
    }

    pub fn get_status(&self, knight: Knight) -> KUsageStatus {
        if let Some(state) = self.0.get(&knight) {
            state.clone()
        } else {
            KUsageStatus::Locked
        }
    }

    pub fn set_status(&mut self, knight: Knight, new_status: KUsageStatus) {
        self.0.insert(knight, new_status);
    }
}

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KnightStatuses::new())
            .add_system(reset_knights.run_if(respawn_message_received));
    }
}

pub fn add_knight_to_tower(
    entity: Entity,
    tower_type: TowerType,
    knight: Knight,
    commands: &mut Commands,
) {
    info!("Adding knight {knight:?} to a tower.");
    let mut e_commands = commands.entity(entity);
    e_commands.insert(knight);

    e_commands
        .insert(Cooldown::new(0.5))
        .insert(Damage(5))
        .insert(Range::new(1.0))
        .insert(Weapon)
        .insert(ProjectileSpawnPoint(Vec3::Y * 2.0));

    match knight {
        Knight::Normal => {
            //TODO: Add appearance changes and generic components
            match tower_type {
                TowerType::Short => {
                    e_commands
                        .insert(PowerSlider::new())
                        .insert(ProjectileSpawnPoint(Vec3::Y * 0.5))
                        .insert(Multishot(10))
                        .insert(Spread(0.25))
                        .insert(Speed(0.2));
                }

                TowerType::Medium => {
                    e_commands.insert(Homing).insert(Range::new(3.0));
                }

                _ => error!("Did not implement tower type: {tower_type:?} for knight: {knight:?}"),
            }
        }
        _ => error!("Did not implement towers for knight: {knight:?}"),
    }
}

fn reset_knights(mut commands: Commands, reset_messages: Query<(Entity, &Reset)>) {
    commands.insert_resource(KnightStatuses::new());

    reset_messages.iter().for_each(|(e, _)| {
        commands.entity(e).insert(IsHandled);
    });
}
