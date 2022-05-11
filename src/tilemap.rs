use crate::prelude::*;

pub enum TileType {
    Empty,
    Rock,
}

pub struct TileModels {
    pub empty: Handle<Scene>,
    pub rock: Handle<Scene>,
}

#[derive(Component)]
pub struct TileMap {
    pub tiles: Vec<Entity>,
    pub width: i32,
    pub height: i32,
}

impl TileMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: Vec::new(),
            width,
            height,
        }
    }

    pub fn initialize_tiles(
        &mut self,
        parent: Entity,
        commands: &mut Commands,
        models: &TileModels,
    ) {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let r_val = thread_rng().gen::<f32>();
                let model = match r_val < 0.8 {
                    true => models.empty.clone(),

                    false => models.rock.clone(),
                };
                let entity = commands
                    .spawn_bundle(TransformBundle::from(Transform {
                        translation: self.calculate_tile_pos(x, y),
                        ..default()
                    }))
                    .insert(Parent(parent))
                    .insert(Name::new(format!("Tile [{x}, {y}]")))
                    .with_children(|p| {
                        p.spawn_scene(model);
                    })
                    .id();

                self.tiles.push(entity);
            });
        });
    }

    fn calculate_tile_pos(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f32 - (self.width as f32) * 0.5,
            0.0,
            y as f32 - (self.height as f32) * 0.5,
        )
    }
}
