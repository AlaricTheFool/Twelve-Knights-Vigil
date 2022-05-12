use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Rock,
    Path,
}

pub struct TileModels {
    pub empty: Handle<Scene>,
    pub rock: Handle<Scene>,
    pub straight: Handle<Scene>,
    pub corner: Handle<Scene>,
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
        let tile_data = self.generate_random_map_layout();
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let tile = tile_data[(x + (y * self.width)) as usize];
                let mut rotation = 0.0;
                let model = match tile {
                    TileType::Empty => models.empty.clone(),
                    TileType::Rock => models.rock.clone(),
                    TileType::Path => {
                        let mut n_path = match self.coord_to_idx(x, y - 1) {
                            Ok(idx) => tile_data[idx] == TileType::Path,
                            Err(_) => false,
                        };
                        let mut s_path = match self.coord_to_idx(x, y + 1) {
                            Ok(idx) => tile_data[idx] == TileType::Path,
                            Err(_) => false,
                        };
                        let mut w_path = match self.coord_to_idx(x - 1, y) {
                            Ok(idx) => tile_data[idx] == TileType::Path,
                            Err(_) => false,
                        };
                        let mut e_path = match self.coord_to_idx(x + 1, y) {
                            Ok(idx) => tile_data[idx] == TileType::Path,
                            Err(_) => false,
                        };

                        let mut result = None;

                        while !result.is_some() {
                            result = match (n_path, s_path, e_path, w_path) {
                                (true, false, true, false) => {
                                    eprintln!("Found a corner match at coords [{x}, {y}] with rotation: {rotation}");
                                    Some(models.corner.clone())},
                                (false, false, true, true) => Some(models.straight.clone()),
                                (false, false, true, false) => Some(models.straight.clone()),
                                _ => None,
                            };
                            let (temp_n, temp_s, temp_e, temp_w) = (n_path, s_path, e_path, w_path);
                            (n_path, s_path, e_path, w_path) = (temp_e, temp_w, temp_s, temp_n);
                            rotation += std::f32::consts::FRAC_PI_2;

                            if rotation > std::f32::consts::PI * 2.0 {
                                panic!("Tried every configuration but found no match. Exits: {n_path}, {e_path}, {s_path}, {w_path}");
                            }
                        }

                        result.unwrap()
                    }
                };

                let entity = commands
                    .spawn_bundle(TransformBundle::from(Transform {
                        translation: self.calculate_tile_pos(x, y),
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
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

    fn generate_random_map_layout(&self) -> Vec<TileType> {
        let mut result = Vec::new();
        result.resize((self.width * self.height) as usize, TileType::Empty);

        let mut current_x = 0;
        let mut current_y = thread_rng().gen_range(1..self.height - 1);
        while current_x < self.width {
            let max_dist_right = 4.min(self.width - current_x);
            let dist_right = thread_rng().gen_range(2.min(max_dist_right)..=max_dist_right);

            (current_x..current_x + dist_right).for_each(|x| {
                result[self.coord_to_idx(x, current_y).unwrap()] = TileType::Path;
            });

            current_x += dist_right;

            if current_x < self.width {
                let new_y = thread_rng().gen_range(1..self.height - 1);

                (current_y.min(new_y)..=new_y.max(current_y)).for_each(|y| {
                    result[self.coord_to_idx(current_x, y).unwrap()] = TileType::Path;
                });

                current_y = new_y;
            }
        }

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let tile = result[self.coord_to_idx(x, y).unwrap()];
                let t_char = match tile {
                    TileType::Path => "x",
                    _ => "_",
                };
                eprint!("{t_char}");
            });
            eprint!("\n");
        });
        result
    }

    fn coord_to_idx(&self, x: i32, y: i32) -> Result<usize, String> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Ok((x + (y * self.width)) as usize)
        } else {
            Err(format!("Index out of Bounds: [{x}, {y}]").to_owned())
        }
    }

    fn calculate_tile_pos(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f32 - (self.width as f32) * 0.5,
            0.0,
            y as f32 - (self.height as f32) * 0.5,
        )
    }
}
