use bevy::{math::vec3, prelude::*};

use crate::card::Card;

use super::{found_set_event::FoundSetEvent, game_manager::GameManager, set::is_set};

#[derive(Component)]
pub struct PlayingField {
    // Represents 3x4 - 3x7 field of cards, None means we want to refill this space
    cards: Vec<Option<Card>>,
}

#[derive(Event)]
pub struct MoveCompressedEvent {}

pub fn display(
    mut commands: Commands,
    mut field_query: Query<&mut GameManager>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut game_manager) = field_query.get_single_mut() {
        let mut cards = game_manager.get_playing_field_mut().get_cards_mut();

        for (id, card) in cards.iter_mut().enumerate() {
            if let Some(card) = card.as_mut() {
                if !card.is_displayed() {
                    card.set_displayed(true);

                    let (x, y, z) = card_id_to_pos(id);

                    commands
                        .spawn((
                            TransformBundle {
                                local: Transform::from_xyz(x, y, z),
                                ..Default::default()
                            },
                            InheritedVisibility::default(),
                            *card,
                        ))
                        .with_children(card.generate_card_entity(&asset_server));
                }
            }
        }
    } else {
        panic!("No game manager found!");
    }
}

pub fn card_id_to_pos(id: usize) -> (f32, f32, f32) {
    const COLUMN_X: &[f32] = &[-200.0, 0.0, 200.0];
    // rows are displayed top to bottom, apart from the *REALLY* rare last row
    const ROW_Y: &[f32] = &[180.0, 60.0, -60.0, -180.0, -300.0, -420.0, 300.0];

    if id > 20 {
        panic!("Unexpected ID!");
    }

    (640.0 + COLUMN_X[id % 3], 450.0 + ROW_Y[id / 3], 0.0)
}

pub fn move_compressed(
    mut field_query: Query<&mut GameManager>,
    mut cards_query: Query<(&mut Card, &mut Transform)>,
    mut move_compressed_event: EventReader<MoveCompressedEvent>,
) {
    for _event in move_compressed_event.read() {
        if let Ok(mut game_manager) = field_query.get_single_mut() {
            let playing_field = game_manager.get_playing_field_mut();
            for (id, card) in playing_field.get_cards().iter().enumerate() {
                if let Some(card) = card {
                    for mut card_entity in cards_query.iter_mut() {
                        if *card == *card_entity.0 {
                            //println!("(MAYBE) MOVING A CARD");
                            let (x, y, z) = card_id_to_pos(id);
                            card_entity.1.translation = vec3(x, y, z);
                        }
                    }
                }
            }
        }
    }
}

pub fn remove_found_set(
    mut commands: Commands,
    mut ev_found_set: EventReader<FoundSetEvent>,
    mut cards: Query<(Entity, &Card)>,
    mut game_manager_query: Query<&mut GameManager>,
    sprites_query: Query<&Sprite>,
    mut ev_move: EventWriter<MoveCompressedEvent>,
) {
    for event in ev_found_set.read() {
        println!("CLEANUP");
        if let Ok(mut game_manager) = game_manager_query.get_single_mut() {
            let set_cards = event.get_cards();

            game_manager.get_playing_field_mut().remove_cards(set_cards);

            println!("Sprites: {}", sprites_query.iter().count());

            for card in &mut cards {
                if card.1.should_remove() {
                    commands.entity(card.0).despawn_recursive();
                }
            }

            println!("Sprites: {}", sprites_query.iter().count());

            game_manager.fill_playing_field();

            ev_move.send(MoveCompressedEvent {});
        }
    }
}

impl PlayingField {
    pub fn new() -> Self {
        PlayingField { cards: vec![] }
    }

    pub fn contains_set(&self) -> bool {
        // it is okay to use brute-force O(n^3) algorithm here because:
        // 1. It is close to optimal (can be redued to O(n^2) using hashmap at best)
        // as the problem is NP-complete (http://pbg.cs.illinois.edu/papers/set.pdf)
        // and there will never be more than 21 cards on the deck at the same time
        // (https://www.sciencedirect.com/science/article/abs/pii/S030402080873322X)
        // therefore we need to check at most 21 choose 3 = 1330 combinations
        // and in most cases 12 choose 3 = 220 combinations

        for i in 0..self.cards.len() {
            for j in i + 1..self.cards.len() {
                for k in j + 1..self.cards.len() {
                    if let (Some(card1), Some(card2), Some(card3)) =
                        (self.cards[i], self.cards[j], self.cards[k])
                    {
                        if is_set(&card1, &card2, &card3) {
                            println!("SET:\n{:?},\n{:?},\n{:?}", card1, card2, card3);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn remove_cards(&mut self, to_remove: Vec<Card>) {
        println!("REMOVING MARKED!");
        for card in self.cards.iter_mut() {
            if let Some(internal) = card {
                if to_remove.contains(internal) {
                    println!("REMOVING A CARD!");
                    *card = None;
                }
            }
        }
    }

    pub fn cards_count(&self) -> usize {
        self.cards.iter().filter(|card| card.is_some()).count()
    }

    pub(crate) fn get_cards(&self) -> Vec<&Option<Card>> {
        self.cards.iter().collect()
    }

    pub(crate) fn get_cards_mut(&mut self) -> Vec<&mut Option<Card>> {
        self.cards.iter_mut().collect()
    }

    // "compress" the playing field, removing any None spaces
    // this will take effect mostly at the end of the game, when there aren't any cards in the deck
    // it will also take effect when we had more than 12 cards on the field at some point because there were no sets
    fn compress(&mut self) {
        // TODO: update the transforms of the cards
        self.cards.retain(|card| card.is_some());
    }

    pub fn add_cards(&mut self, mut added: Vec<Card>) {
        // Add cards, beginning with filling the empty spaces and then appending to the end of the vector
        for card in self.cards.iter_mut() {
            if card.is_some() {
                continue;
            }
            if let Some(new_card) = added.pop() {
                println!("FILLING EMPTY SPACE WITH CARD");
                *card = Some(new_card);
            }
        }

        self.compress();

        self.cards.extend(added.into_iter().map(Some));
    }
}
