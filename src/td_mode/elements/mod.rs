mod chemistry;

use super::*;
use std::collections::HashMap;
use std::ops::{Add, Sub, SubAssign};

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

    pub fn subtract_element(&mut self, element: Element, amount: u32) {
        let initial = if let Some(&existing_val) = self.0.get(&element) {
            existing_val
        } else {
            0
        };

        self.0.insert(element, initial.saturating_sub(amount));
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

    /// Checks if all the elements in other are present in self
    pub fn contains(&self, other: &ElementalAffliction) -> bool {
        other
            .0
            .iter()
            .all(|(&element, &amount)| self.get_element_amount(element) >= amount)
    }

    /// Checks if all the elements in other have the exact same values as self.
    pub fn contains_exactly(&self, other: &ElementalAffliction) -> bool {
        other
            .0
            .iter()
            .all(|(&element, &amount)| self.get_element_amount(element) == amount)
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

impl Sub<&ElementalAffliction> for &ElementalAffliction {
    type Output = ElementalAffliction;

    fn sub(self, other: &ElementalAffliction) -> Self::Output {
        let mut result = self.clone();

        other.0.iter().for_each(|(&element, &amount)| {
            result.subtract_element(element, amount);
        });

        result
    }
}

impl SubAssign<&ElementalAffliction> for ElementalAffliction {
    fn sub_assign(&mut self, other: &ElementalAffliction) {
        other.0.iter().for_each(|(&element, &amount)| {
            self.subtract_element(element, amount);
        });
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
pub struct ApplyElement;

/// Tag component for reducing elements
#[derive(Component)]
pub struct RemoveElements;

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

const CONSOLIDATE_MESSAGE_STAGE: &str = "consolidate_messages";
const APPLY_ELEMENT_STAGE: &str = "apply_elements";
const REMOVE_ELEMENT_STAGE: &str = "remove_elements";

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(chemistry::ChemistryPlugin)
            .add_stage_before(
                CoreStage::Update,
                APPLY_ELEMENT_STAGE,
                SystemStage::parallel(),
            )
            .add_stage_before(
                APPLY_ELEMENT_STAGE,
                REMOVE_ELEMENT_STAGE,
                SystemStage::parallel(),
            )
            .add_stage_before(
                REMOVE_ELEMENT_STAGE,
                CONSOLIDATE_MESSAGE_STAGE,
                SystemStage::parallel(),
            )
            .add_system_to_stage(
                CONSOLIDATE_MESSAGE_STAGE,
                consolidate_element_messages.run_in_state(GameState::TDMode),
            )
            .add_system_to_stage(
                REMOVE_ELEMENT_STAGE,
                handle_remove_element_messages.run_in_state(GameState::TDMode),
            )
            .add_system_to_stage(
                APPLY_ELEMENT_STAGE,
                handle_apply_element_messages.run_in_state(GameState::TDMode),
            );
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
                trace!("Adding to existing affliction");
                commands
                    .entity(target.0)
                    .insert(existing_affliction + affliction);
            } else {
                trace!("Creating new affliction.");
                commands.entity(target.0).insert(affliction.clone());
            }

            // Message is handled
            commands.entity(message_entity).insert(Handled);
        });
}

fn handle_remove_element_messages(
    message_query: Query<
        (Entity, &Target, &ElementalAffliction),
        (With<Message>, With<RemoveElements>),
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
                    .insert(existing_affliction - affliction);
            }
            // Message is handled
            commands.entity(message_entity).insert(Handled);
        });
}

fn consolidate_element_messages(
    message_query: Query<
        (
            Entity,
            &Target,
            &ElementalAffliction,
            Option<&ApplyElement>,
            Option<&RemoveElements>,
        ),
        (
            With<Message>,
            Or<(With<ApplyElement>, With<RemoveElements>)>,
        ),
    >,
    mut commands: Commands,
) {
    let mut apply_sums = HashMap::new();
    let mut remove_sums = HashMap::new();

    let mut all_added = ElementalAffliction::empty();
    let mut all_subtracted = ElementalAffliction::empty();
    // Sum up existing messages and despawn them
    message_query
        .iter()
        .for_each(|(entity, target, elements, apply, remove)| {
            if apply.is_some() {
                all_added = elements + &all_added;
                if let Some(existing_elements) = apply_sums.remove(&target.0) {
                    apply_sums.insert(target.0, &existing_elements + elements);
                } else {
                    apply_sums.insert(target.0, elements.clone());
                }
            } else if remove.is_some() {
                all_subtracted = &all_subtracted + elements;
                if let Some(existing_elements) = remove_sums.remove(&target.0) {
                    remove_sums.insert(target.0, &existing_elements + elements);
                } else {
                    remove_sums.insert(target.0, elements.clone());
                }
            }

            commands.entity(entity).despawn_recursive();
        });

    // Spawn new message for each target with totals

    apply_sums.iter().for_each(|(target, elements)| {
        trace!("Adding {elements} to {target:?}");
        commands
            .spawn()
            .insert(Message)
            .insert(Target(*target))
            .insert(ApplyElement)
            .insert(elements.clone());
    });

    remove_sums.iter().for_each(|(target, elements)| {
        trace!("Removing {elements} from {target:?}");
        commands
            .spawn()
            .insert(Message)
            .insert(Target(*target))
            .insert(RemoveElements)
            .insert(elements.clone());
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
    fn subtract_elemental_affliction() {
        let mut first = ElementalAffliction::empty();
        let mut second = ElementalAffliction::empty();
        let mut result = ElementalAffliction::empty();

        first.add_element(Element::Air, 9);
        second.add_element(Element::Air, 6);

        result.add_element(Element::Air, 3);

        assert_eq!(&first - &second, result);
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
