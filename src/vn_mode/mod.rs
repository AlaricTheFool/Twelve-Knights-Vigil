use crate::prelude::*;

mod parser;
mod scene;

pub use parser::*;
pub use scene::*;

pub struct VNModePlugin;

impl Plugin for VNModePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VNScene::new());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VNEvent {
    Dialogue(Speaker, String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Speaker {
    name: String,
}

impl Speaker {
    pub fn from_knight(knight: Knight) -> Self {
        Self {
            name: knight.get_name().to_string(),
        }
    }

    pub fn player() -> Self {
        Self {
            name: "Me".to_string(),
        }
    }

    pub fn narrator() -> Self {
        Self {
            name: "Narrator".to_string(),
        }
    }

    pub fn from_key(key: &str) -> Result<Self, String> {
        match key {
            "PLAYER" => Ok(Self::player()),
            "NARRATOR" => Ok(Self::narrator()),
            "AVERAGE_KNIGHT" => Ok(Self::from_knight(Knight::Normal)),
            _ => Err("Unrecognized Character Code: {key}".to_string()),
        }
    }
}
