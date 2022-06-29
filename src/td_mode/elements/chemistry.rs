use super::*;
use map::{Tile, TileType};

struct ChemicalReaction {
    tile_type: TileType,
    prerequisites: ElementalAffliction,
    new_tile_type: Option<TileType>,
    // TODO: OPTIONAL MESSAGE TO SEND
}

struct Reactions(Vec<ChemicalReaction>);

pub struct ChemistryPlugin;

impl Plugin for ChemistryPlugin {
    fn build(&self, app: &mut App) {
        let fire_to_rock_prereq = ElementalAffliction::single(Element::Water, 20);
        let fire_to_rock = ChemicalReaction {
            tile_type: TileType::Fire,
            prerequisites: fire_to_rock_prereq,
            new_tile_type: Some(TileType::Rock),
        };

        let rock_to_fire_prereq = ElementalAffliction::single(Element::Fire, 50);
        let rock_to_fire = ChemicalReaction {
            tile_type: TileType::Rock,
            prerequisites: rock_to_fire_prereq,
            new_tile_type: Some(TileType::Fire),
        };

        let reactions = Reactions(vec![rock_to_fire, fire_to_rock]);
        app.insert_resource(reactions);

        app.add_system(trigger_reactions.run_in_state(GameState::TDMode));
    }
}

fn trigger_reactions(
    mut tile_query: Query<
        (&mut TileType, &mut ElementalAffliction),
        (With<Tile>, Changed<ElementalAffliction>),
    >,
    reactions: Res<Reactions>,
) {
    tile_query
        .iter_mut()
        .for_each(|(mut tile_type, mut affliction)| {
            reactions.0.iter().for_each(|reaction| {
                // CHECK IF AFFLICTION MEETS REQS
                if tile_type.clone() == reaction.tile_type
                    && affliction.contains(&reaction.prerequisites)
                {
                    // CHANGE TILETYPE IF NECESSARY
                    if let Some(new_tile_type) = reaction.new_tile_type {
                        *tile_type = new_tile_type;
                    }

                    // SPAWN AN EVENT MESSAGE IF NECESSARY
                }
            });
        });
}
