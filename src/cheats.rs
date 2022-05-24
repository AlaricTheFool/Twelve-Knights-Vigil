use crate::prelude::*;

pub struct CheatPlugin;

impl Plugin for CheatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cheats::new());
    }
}

pub struct Cheats {
    pub infinite_lives: bool,
    pub infinite_money: bool,
}

impl Cheats {
    fn new() -> Self {
        Self {
            infinite_lives: cfg!(feature = "bulletproof"),
            infinite_money: cfg!(feature = "screw_the_rules"),
        }
    }
}
