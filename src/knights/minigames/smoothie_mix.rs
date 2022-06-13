use super::*;

#[derive(Component, Clone)]
pub struct SmoothieMix {
    game_state: GameState,
    recipe: Vec<CookingActions>,
    current_mix: Vec<CookingActions>,
}

impl SmoothieMix {
    pub fn new() -> Self {
        Self {
            game_state: GameState::Mixing,
            recipe: Vec::new(),
            current_mix: Vec::new(),
        }
    }

    fn with_state(&self, state: GameState) -> Self {
        let mut result = self.clone();
        result.game_state = state;
        result
    }
}

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Mixing,
    ReadingRecipe,
    Finished,
}

#[derive(Clone, Copy, PartialEq)]
enum CookingActions {
    AddIngredient(Ingredient),
}

#[derive(Clone, Copy, PartialEq)]
enum Ingredient {
    ProteinPowder,
    Banana,
    Oats,
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

        if show_recipe {
            ui.label("First do the thing\nThen do the other thing\nThen do the next thing");
        } else {
            ui.horizontal(|ui| {
                ui.button("Protein Powder");
                ui.button("Banana");
                ui.button("Oats");
            });

            ui.horizontal(|ui| {
                ui.button("Shake");
                ui.button("Stir");
                ui.button("Blend");
            });

            ui.button("Serve");
            ui.button("Start Over");
        }

        None
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
