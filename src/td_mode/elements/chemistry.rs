use super::*;
use map::{Tile, TileType};
use std::ops::ControlFlow;

struct ChemicalReaction {
    tile_type: TileType,
    prerequisites: ElementalAffliction,
    prereq_type: PrerequisiteType,
    subtract_prerequisites: bool,
    new_tile_type: Option<TileType>,
    // TODO: OPTIONAL MESSAGE TO SEND
}

enum PrerequisiteType {
    Contains,
    ExactMatch,
}

impl ChemicalReaction {
    fn check_prereqs_against(&self, other: &ElementalAffliction) -> bool {
        match self.prereq_type {
            PrerequisiteType::Contains => other.contains(&self.prerequisites),
            PrerequisiteType::ExactMatch => other.contains_exactly(&self.prerequisites),
        }
    }
}

struct Reactions(Vec<ChemicalReaction>);

pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App) {
        let fire_to_rock_prereq = ElementalAffliction::single(Element::Water, 20);
        let fire_to_rock = ChemicalReaction {
            tile_type: TileType::Fire,
            prerequisites: fire_to_rock_prereq,
            prereq_type: PrerequisiteType::Contains,
            subtract_prerequisites: true,
            new_tile_type: Some(TileType::Rock),
        };

        let rock_to_fire_prereq = ElementalAffliction::single(Element::Fire, 50);
        let rock_to_fire = ChemicalReaction {
            tile_type: TileType::Rock,
            prerequisites: rock_to_fire_prereq,
            prereq_type: PrerequisiteType::Contains,
            subtract_prerequisites: false,
            new_tile_type: Some(TileType::Fire),
        };

        let fire_cooling_prereq = ElementalAffliction::single(Element::Fire, 0);
        let fire_cooling = ChemicalReaction {
            tile_type: TileType::Fire,
            prerequisites: fire_cooling_prereq,
            prereq_type: PrerequisiteType::ExactMatch,
            subtract_prerequisites: false,
            new_tile_type: Some(TileType::Rock),
        };

        let reactions = Reactions(vec![rock_to_fire, fire_to_rock, fire_cooling]);
        app.insert_resource(reactions);

        app.add_system(trigger_reactions.run_in_state(GameState::TDMode));
    }
}

fn trigger_reactions(
    mut tile_query: Query<
        (&Coordinate, &TileType, &mut ElementalAffliction),
        (With<Tile>, Changed<ElementalAffliction>),
    >,
    reactions: Res<Reactions>,
    mut map: ResMut<Map>,
) {
    tile_query
        .iter_mut()
        .for_each(|(coord, tile_type, mut affliction)| {
            reactions.0.iter().try_for_each(|reaction| {
                // CHECK IF AFFLICTION MEETS REQS
                if *tile_type == reaction.tile_type && reaction.check_prereqs_against(&affliction) {
                    // CHANGE TILETYPE IF NECESSARY
                    if let Some(new_tile_type) = reaction.new_tile_type {
                        map.set_tile(*coord, Some(new_tile_type), None);
                    }

                    // SPAWN AN EVENT MESSAGE IF NECESSARY

                    // Subtract the elements used in the reaction
                    if reaction.subtract_prerequisites {
                        *affliction -= &reaction.prerequisites;
                    }
                    return ControlFlow::Break(());
                }

                ControlFlow::Continue(())
            });
        });
}
