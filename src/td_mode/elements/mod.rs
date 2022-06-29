mod chemistry;

use super::*;
use std::collections::HashMap;
use std::ops::Add;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Element {
    Fire,
    Water,
    Earth,
    Air,
}

impl Element {
    pub fn all() -> [Self; 4] {
        [Self::Fire, Self::Water, Self::Earth, Self::Air]
    }

    fn display_name(&self) -> &str {
        match *self {
            Self::Fire => "Fire",
            Self::Water => "Water",
            Self::Earth => "Earth",
            Self::Air => "Air",
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Component, PartialEq, Debug, Clone)]
pub struct ElementalAffliction(HashMap<Element, u32>);

impl ElementalAffliction {
    pub fn empty() -> Self {
        ElementalAffliction(HashMap::new())
    }

    pub fn add_element(&mut self, element: Element, amount: u32) {
        let initial = if let Some(&existing_val) = self.0.get(&element) {
            existing_val
        } else {
            0
        };

        self.0.insert(element, initial + amount);
    }

    pub fn single(element: Element, amount: u32) -> Self {
        let mut result = Self::empty();
        result.add_element(element, amount);

        result
    }

    pub fn get_element_amount(&self, element: Element) -> u32 {
        if let Some(&val) = self.0.get(&element) {
            val
        } else {
            0
        }
    }

    pub fn contains(&self, other: &ElementalAffliction) -> bool {
        other
            .0
            .iter()
            .all(|(&element, &amount)| self.get_element_amount(element) >= amount)
    }
}

impl Add<&ElementalAffliction> for &ElementalAffliction {
    type Output = ElementalAffliction;

    fn add(self, other: &ElementalAffliction) -> Self::Output {
        let mut result = self.clone();

        other.0.iter().for_each(|(&element, &amount)| {
            result.add_element(element, amount);
        });

        result
    }
}

impl std::fmt::Display for ElementalAffliction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let element_count = self.0.keys().len();

        self.0
            .iter()
            .enumerate()
            .try_for_each(|(idx, (element, amount))| {
                write!(f, "{element}: {amount}")?;

                if idx < element_count - 1 {
                    write!(f, "\n")?;
                }

                Ok(())
            })
    }
}

/// Tag component for applying elements
#[derive(Component)]
struct ApplyElement;

#[derive(Bundle)]
pub struct ApplyElementMessage {
    element: ElementalAffliction,
    target: Target,
    message: Message,
    apply_element: ApplyElement,
}

impl ApplyElementMessage {
    pub fn single_element(element: Element, amount: u32, target: Entity) -> Self {
        Self {
            element: ElementalAffliction::single(element, amount),
            target: Target(target),
            message: Message,
            apply_element: ApplyElement,
        }
    }
}

/// This plugin handles all elemental interactions
pub struct ElementPlugin;

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(chemistry::ChemistryPlugin)
            .add_system(handle_apply_element_messages.run_in_state(GameState::TDMode));
    }
}

fn handle_apply_element_messages(
    message_query: Query<
        (Entity, &Target, &ElementalAffliction),
        (With<Message>, With<ApplyElement>),
    >,
    affliction_query: Query<&ElementalAffliction>,
    mut commands: Commands,
) {
    message_query
        .iter()
        .for_each(|(message_entity, target, affliction)| {
            //DO THE STUFF
            if let Ok(existing_affliction) = affliction_query.get(target.0) {
                commands
                    .entity(target.0)
                    .insert(existing_affliction + affliction);
            } else {
                commands.entity(target.0).insert(affliction.clone());
            }

            // Message is handled
            commands.entity(message_entity).insert(Handled);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_elemental_afflictions_together() {
        let mut first = ElementalAffliction::empty();
        let mut second = ElementalAffliction::empty();
        let mut result = ElementalAffliction::empty();

        first.add_element(Element::Air, 3);
        result.add_element(Element::Air, 3);

        second.add_element(Element::Earth, 6);
        result.add_element(Element::Earth, 6);

        assert_eq!(&first + &second, result);
    }

    #[test]
    fn add_element_to_affliction() {
        let mut affliction = ElementalAffliction::empty();

        affliction.add_element(Element::Air, 3);

        let expected = 3;
        let actual = affliction.get_element_amount(Element::Air);
        assert_eq!(expected, actual);

        affliction.add_element(Element::Air, 6);

        let expected = 3 + 6;
        let actual = affliction.get_element_amount(Element::Air);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_afflictions_contain_other_afflictions() {
        let mut parent = ElementalAffliction::empty();
        let mut child = ElementalAffliction::empty();

        // Empty contains empty
        assert!(parent.contains(&child));

        // Child contains one and parent is empty
        child.add_element(Element::Earth, 4);
        assert_eq!(false, parent.contains(&child));

        // Parent and Child Single Element
        parent.add_element(Element::Earth, 4);
        assert!(parent.contains(&child));

        // Parent has more than child
        parent.add_element(Element::Earth, 4);
        assert!(parent.contains(&child));

        // Parent has other elements
        parent.add_element(Element::Water, 5);
        assert!(parent.contains(&child));
    }
}
