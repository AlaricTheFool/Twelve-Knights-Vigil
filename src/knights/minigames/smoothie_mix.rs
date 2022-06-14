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
}

impl CookingAction {
    fn generate_random_recipe(steps: usize) -> Vec<CookingAction> {
        (0..steps)
            .map(|_| CookingAction::AddIngredient(Ingredient::ProteinPowder))
            .collect()
    }

    fn to_recipe_line(&self, count: usize) -> String {
        format!("Add {} tablespoon(s) of protein powder.", count)
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
                ui.button("Shake");
                ui.button("Stir");
                ui.button("Blend");
            });

            ui.button("Serve");
            ui.button("Start Over");
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
