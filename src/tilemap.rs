use crate::prelude::*;

pub struct CurrentMap(pub Option<Entity>);

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub enum TileType {
    Empty,
    Rock,
    Tree,
    Path(PathType, f32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PathType {
    Straight,
    Corner,
}

pub struct TileModels {
    pub empty: Handle<Scene>,
    pub rock: Handle<Scene>,
    pub straight: Handle<Scene>,
    pub corner: Handle<Scene>,
    pub tree: Handle<Scene>,
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
        let mut tile_parent = parent;
        commands.entity(parent).with_children(|p| {
            tile_parent = p
                .spawn()
                .insert_bundle(TransformBundle::identity())
                .insert(Name::new("Tiles"))
                .id();
        });
        let (tile_data, path_points) = self.generate_random_map_layout();
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let tile = tile_data[(x + (y * self.width)) as usize];
                let (model, rotation) = match tile {
                    TileType::Empty => (models.empty.clone(), 0.0),
                    TileType::Rock => (models.rock.clone(), 0.0),
                    TileType::Path(p_type, rot) => {
                        let m = match p_type {
                            PathType::Straight => models.straight.clone(),
                            PathType::Corner => models.corner.clone(),
                        };
                        (m, rot)
                    }
                    TileType::Tree => (models.tree.clone(), 0.0),
                };

                let entity = commands
                    .spawn()
                    .insert_bundle(TransformBundle::from(Transform {
                        translation: self.calculate_tile_pos(x, y),
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
                        ..default()
                    }))
                    .insert(Parent(tile_parent))
                    .insert(Name::new(format!("Tile [{x}, {y}]")))
                    .insert(Tile { x, y })
                    .insert(tile)
                    .with_children(|p| {
                        p.spawn_scene(model);
                    })
                    .id();

                self.tiles.push(entity);
            });
        });

        commands
            .get_or_spawn(parent)
            .insert(Track::new(path_points));
    }

    fn generate_random_map_layout(&self) -> (Vec<TileType>, Vec<Transform>) {
        let mut result = Vec::new();
        let mut path_points = Vec::new();
        result.resize((self.width * self.height) as usize, TileType::Empty);

        let mut current_x = 0;
        let mut current_y = thread_rng().gen_range(1..self.height - 1);
        let mut just_went_up = false;

        path_points.push(Transform::from_translation(
            self.calculate_tile_pos(current_x - 1, current_y),
        ));
        while current_x < self.width {
            let max_dist_right = 4.min(self.width - current_x);
            let dist_right = thread_rng().gen_range(2.min(max_dist_right)..=max_dist_right);

            (current_x..current_x + dist_right).for_each(|x| {
                let corner_rot = match just_went_up {
                    true => std::f32::consts::PI,
                    false => -std::f32::consts::FRAC_PI_2,
                };
                result[self.coord_to_idx(x, current_y).unwrap()] = match x {
                    x if x == current_x && current_x != 0 => {
                        TileType::Path(PathType::Corner, corner_rot)
                    }

                    x if x == current_x + dist_right => {
                        TileType::Path(PathType::Corner, corner_rot)
                    }

                    _ => TileType::Path(PathType::Straight, std::f32::consts::FRAC_PI_2),
                };
            });

            current_x += dist_right;
            path_points.push(Transform::from_translation(
                self.calculate_tile_pos(current_x, current_y),
            ));

            if current_x < self.width {
                let mut new_y = current_y;

                while new_y == current_y {
                    new_y = thread_rng().gen_range(1..self.height - 1);
                }

                (current_y.min(new_y)..=new_y.max(current_y))
                    .enumerate()
                    .for_each(|(idx, y)| {
                        result[self.coord_to_idx(current_x, y).unwrap()] =
                            match (new_y > current_y, idx) {
                                (true, 0) => {
                                    TileType::Path(PathType::Corner, std::f32::consts::FRAC_PI_2)
                                }

                                (false, i) if i == (current_y - new_y) as usize => {
                                    TileType::Path(PathType::Corner, 0.0)
                                }

                                _ => TileType::Path(PathType::Straight, 0.0),
                            };
                    });
                just_went_up = new_y < current_y;

                current_y = new_y;
                path_points.push(Transform::from_translation(
                    self.calculate_tile_pos(current_x, current_y),
                ));
            }
        }

        result = result
            .iter()
            .map(|tile| match *tile {
                TileType::Empty => {
                    if thread_rng().gen::<f32>() < 0.9 {
                        *tile
                    } else {
                        if thread_rng().gen() {
                            TileType::Tree
                        } else {
                            TileType::Rock
                        }
                    }
                }
                _ => *tile,
            })
            .collect();

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let tile = result[self.coord_to_idx(x, y).unwrap()];
                let t_char = match tile {
                    TileType::Path(_, _) => "x",
                    _ => "_",
                };
                eprint!("{t_char}");
            });
            eprint!("\n");
        });
        (result, path_points)
    }

    fn coord_to_idx(&self, x: i32, y: i32) -> Result<usize, String> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Ok((x + (y * self.width)) as usize)
        } else {
            Err(format!("Index out of Bounds: [{x}, {y}]").to_owned())
        }
    }

    pub fn get_tile_entity_at_coord(&self, coord: Coordinate) -> Result<Entity, String> {
        match self.coord_to_idx(coord.x, coord.y) {
            Ok(idx) => Ok(self.tiles[idx]),

            Err(msg) => Err(msg),
        }
    }

    pub fn calculate_tile_pos(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f32 - (self.width as f32) * 0.5,
            0.0,
            y as f32 - (self.height as f32) * 0.5,
        )
    }
}

#[derive(Component)]
pub struct Track {
    pub points: Vec<Transform>,
    pub length: f32,
}

impl Track {
    pub fn new(points: Vec<Transform>) -> Self {
        let length = points
            .iter()
            .enumerate()
            .skip(1)
            .fold(0.0, |acc, (idx, point)| {
                let last_point = points[idx - 1];
                let dist = point.translation.distance(last_point.translation);
                acc + dist
            });

        Self { points, length }
    }
    pub fn get_point(&self, progress: f32) -> Transform {
        let mut remaining_progress = progress;
        self.points
            .iter()
            .enumerate()
            .find_map(|(idx, point)| {
                if idx == self.points.len() - 1 {
                    return Some(*point);
                };
                let next_point = self.points[idx + 1];
                let dist = next_point.translation.distance(point.translation);

                if remaining_progress > dist {
                    remaining_progress -= dist;
                    None
                } else {
                    let pct = remaining_progress / dist;
                    let pos = point.translation.lerp(next_point.translation, pct);
                    Some(Transform::from_translation(pos))
                }
            })
            .expect("Tried to get a point on an empty track.")
    }
}
