use super::*;
use smoothie_mix::*;

#[derive(Bundle)]
pub struct MuscleKnightBaseBundle {
    powerbar: PowerBar,
    smoothie_mix: SmoothieMix,
}

impl MuscleKnightBaseBundle {
    pub fn new() -> Self {
        Self {
            powerbar: PowerBar::new(),
            smoothie_mix: SmoothieMix::new(),
        }
    }
}
