use crate::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct MessagePlugin;

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<BuildTower>();
        }
    }
}

#[derive(Component)]
pub struct Message;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct BuildTower {
    pub location: Coordinate,
}
