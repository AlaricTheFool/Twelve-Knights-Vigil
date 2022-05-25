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
        app.insert_resource(KnightStatuses::new());
    }
}

pub fn add_knight_to_tower(entity: Entity, knight: Knight, commands: &mut Commands) {
    trace!("Adding knight {knight:?} to a tower.");
    commands.entity(entity).insert(knight);
}
