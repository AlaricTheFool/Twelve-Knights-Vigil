use super::*;
use crate::knights::tic_tac_toe::*;

#[derive(Bundle)]
pub struct NormalKnightBaseBundle {
    cd: Cooldown,
    damage: Damage,
    weapon: Weapon,
    powerbar: PowerBar,
    tic_tac_toe: TicTacToe,
}

impl NormalKnightBaseBundle {
    fn new() -> Self {
        Self {
            cd: Cooldown::new(0.5),
            damage: Damage(5),
            weapon: Weapon,
            powerbar: PowerBar::new(),
            tic_tac_toe: TicTacToe::new(),
        }
    }
}

#[derive(Bundle)]
pub struct NormalKnightShortTowerBundle {
    #[bundle]
    base: NormalKnightBaseBundle,
    spawn_point: ProjectileSpawnPoint,
    range: Range,
    multishot: Multishot,
}

impl NormalKnightShortTowerBundle {
    fn new() -> Self {
        Self {
            base: NormalKnightBaseBundle::new(),
            spawn_point: ProjectileSpawnPoint(Vec3::Y * 0.5),
            range: Range::new(1.0),
            multishot: Multishot(10),
        }
    }
}

#[derive(Bundle)]
pub struct NormalKnightMediumTowerBundle {
    #[bundle]
    base: NormalKnightBaseBundle,
    homing: Homing,
    spawn_point: ProjectileSpawnPoint,
    range: Range,
}

impl NormalKnightMediumTowerBundle {
    fn new() -> Self {
        Self {
            base: NormalKnightBaseBundle::new(),
            homing: Homing,
            range: Range::new(3.0),
            spawn_point: ProjectileSpawnPoint(Vec3::Y * 2.0),
        }
    }
}

#[derive(Bundle)]
pub struct NormalKnightTallTowerBundle {
    #[bundle]
    base: NormalKnightBaseBundle,
    explosive: Explosive,
    range: Range,
    spawn_point: ProjectileSpawnPoint,
}

impl NormalKnightTallTowerBundle {
    fn new() -> Self {
        Self {
            base: NormalKnightBaseBundle::new(),
            explosive: Explosive(Range::new(0.5)),
            spawn_point: ProjectileSpawnPoint(Vec3::Y * 2.0),
            range: Range::new(1.0),
        }
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

    match (knight, tower_type) {
        (Knight::Normal, TowerType::Short) => {
            e_commands.insert_bundle(NormalKnightShortTowerBundle::new());
        }

        (Knight::Normal, TowerType::Medium) => {
            e_commands.insert_bundle(NormalKnightMediumTowerBundle::new());
        }

        (Knight::Normal, TowerType::Tall) => {
            e_commands.insert_bundle(NormalKnightTallTowerBundle::new());
        }

        _ => error!("Did not implement tower type: {tower_type:?} for knight: {knight:?}"),
    }
}

pub fn remove_knight_from_tower(
    entity: Entity,
    tower_type: TowerType,
    knight: Knight,
    commands: &mut Commands,
) {
    info!("Removing knight {knight:?} from a tower.");
    let mut e_commands = commands.entity(entity);
    e_commands.remove::<Knight>();

    match (knight, tower_type) {
        (Knight::Normal, TowerType::Short) => {
            e_commands.remove_bundle::<NormalKnightShortTowerBundle>();
        }

        (Knight::Normal, TowerType::Medium) => {
            e_commands.remove_bundle::<NormalKnightMediumTowerBundle>();
        }

        (Knight::Normal, TowerType::Tall) => {
            e_commands.remove_bundle::<NormalKnightTallTowerBundle>();
        }

        _ => error!("Did not implement tower type: {tower_type:?} for knight: {knight:?}"),
    }
}
