use super::*;

#[derive(Component, Clone)]
pub struct SmoothieMix {
    game_state: GameState,
    recipe: Vec<CookingAction>,
    current_mix: Vec<CookingAction>,
}

impl SmoothieMix {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Mixing,
            recipe: CookingAction::generate_random_recipe(3),
            current_mix: Vec::new(),
        }
    }

    fn with_state(&self, state: GameState) -> Self {
        let mut result = self.clone();
        result.game_state = state;
        result
    }

    fn with_added_action(&self, action: CookingAction) -> Self {
        let mut result = self.clone();
        result.current_mix.push(action);
        result
    }

    fn get_recipe_text(&self) -> String {
        self.recipe
            .iter()
            .map(|action| action.to_recipe_line(1))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn with_empty_mix(&self) -> Self {
        let mut res = self.clone();
        res.current_mix = Vec::new();
        res
    }
}

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Mixing,
    ReadingRecipe,
    Finished,
}

#[derive(Clone, Copy, PartialEq)]
enum CookingAction {
    AddIngredient(Ingredient),
    UseTechnique(Technique),
}

impl CookingAction {
    fn generate_random_recipe(steps: usize) -> Vec<CookingAction> {
        use rand::seq::SliceRandom;
        (0..steps)
            .map(|_| match thread_rng().gen_range(0..=1) {
                0 => {
                    let mut ingredients = Ingredient::all();
                    ingredients.shuffle(&mut thread_rng());
                    let ingredient = ingredients[0];
                    CookingAction::AddIngredient(ingredient)
                }
                1 => {
                    let mut techniques = Technique::all();
                    techniques.shuffle(&mut thread_rng());
                    let technique = techniques[0];
                    CookingAction::UseTechnique(technique)
                }
                _ => {
                    unreachable!()
                }
            })
            .collect()
    }

    fn to_recipe_line(&self, count: usize) -> String {
        match *self {
            CookingAction::AddIngredient(ingredient) => match ingredient {
                Ingredient::ProteinPowder => format!("Add {count} tablespoons of Protein Powder"),
                Ingredient::Banana => format!("Add {count} bananas"),
                Ingredient::Oats => format!("Add {count} cups of oats"),
            },

            CookingAction::UseTechnique(technique) => match technique {
                Technique::Blend => format!("Run Blender {count} times"),
                Technique::Shake => format!("Shake Firmly {count} times"),
                Technique::Stir => format!("Stir {count} times"),
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Ingredient {
    ProteinPowder,
    Banana,
    Oats,
}

impl Ingredient {
    fn all() -> [Ingredient; 3] {
        [
            Ingredient::ProteinPowder,
            Ingredient::Banana,
            Ingredient::Oats,
        ]
    }

    fn display_name(&self) -> &str {
        match *self {
            Ingredient::ProteinPowder => "Protein Powder",
            Ingredient::Banana => "Banana",
            Ingredient::Oats => "Oats",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Technique {
    Blend,
    Shake,
    Stir,
}

impl Technique {
    fn all() -> [Technique; 3] {
        [Technique::Blend, Technique::Shake, Technique::Stir]
    }

    fn display_name(&self) -> &str {
        match *self {
            Technique::Blend => "Blend",
            Technique::Shake => "Shake",
            Technique::Stir => "Stir",
        }
    }
}

impl KnightMinigame for SmoothieMix {
    fn render_egui(&self, ui: &mut Ui) -> Option<Self> {
        let mut show_recipe = self.game_state == GameState::ReadingRecipe;
        if ui.checkbox(&mut show_recipe, "Show Recipe").clicked() {
            if show_recipe {
                return Some(self.with_state(GameState::ReadingRecipe));
            } else {
                return Some(self.with_state(GameState::Mixing));
            }
        }

        let mut result = None;
        if show_recipe {
            ui.label(self.get_recipe_text());
        } else {
            ui.horizontal(|ui| {
                Ingredient::all().iter().for_each(|ingredient| {
                    if ui.button(ingredient.display_name()).clicked() {
                        result =
                            Some(self.with_added_action(CookingAction::AddIngredient(*ingredient)));
                    }
                })
            });

            ui.horizontal(|ui| {
                Technique::all().iter().for_each(|technique| {
                    if ui.button(technique.display_name()).clicked() {
                        result =
                            Some(self.with_added_action(CookingAction::UseTechnique(*technique)));
                    }
                })
            });

            if ui.button("Serve").clicked() {
                result = Some(self.with_state(GameState::Finished));
            }
            if ui.button("Start Over").clicked() {
                result = Some(self.with_empty_mix());
            }
        }

        result
    }

    fn get_game_result(&self) -> Option<GameResult> {
        if self.game_state == GameState::Finished {
            if self.recipe.iter().eq(self.current_mix.iter()) {
                Some(GameResult::Victory)
            } else {
                Some(GameResult::Defeat)
            }
        } else {
            None
        }
    }
}
