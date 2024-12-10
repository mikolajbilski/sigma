use bevy::prelude::*;

use crate::{card::Card, mechanics::playing_field};

use super::{game_manager::GameManager, set::is_set};

#[derive(Component)]
pub struct PlayingField {
    // Represents 3x4 - 3x7 field of cards, None means we want to refill this space
    cards: Vec<Option<Card>>,
}

pub fn display(
    mut commands: Commands,
    field_query: Query<&GameManager>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(game_manager) = field_query.get_single() {
        let playing_field = game_manager.get_playing_field();
        let cards_count = playing_field.cards_count();
        let rows = cards_count / 3;

        const COLUMN_X: &[f32] = &[-250.0, 0.0, 250.0];
        // rows are displayed top to bottom, apart from the *REALLY* rare last row
        const ROW_Y: &[f32] = &[240.0, 80.0, -80.0, -240.0, -400.0, -560.0, 400.0];

        let cards = playing_field.get_cards();

        for row_no in 0..rows {
            for column_no in 0..3 {
                let card_id = 3 * row_no + column_no;
                if card_id < cards_count {
                    if let Some(card) = cards[card_id] {
                        println!("Row: {row_no}, column: {column_no}, card: {card_id}");
                        println!("x: {}, y: {}, card: {card_id}", 600.0 + ROW_Y[row_no], 400.0 + COLUMN_X[column_no]);
                        // Display this card
                        commands
                            .spawn(TransformBundle {
                                local: Transform::from_xyz(
                                    600.0 + COLUMN_X[column_no],
                                    400.0 + ROW_Y[row_no],
                                    0.0,
                                ),
                                ..Default::default()
                            })
                            .with_children(card.generate_card_entity(&asset_server));
                    }
                }
            }
        }
    } else {
        println!("No game manager found!");
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
            if let Some(card1) = self.cards[i] {
                for j in i + 1..self.cards.len() {
                    if let Some(card2) = self.cards[j] {
                        for k in j + 1..self.cards.len() {
                            if let Some(card3) = self.cards[k] {
                                if is_set(&card1, &card2, &card3) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn cards_count(&self) -> usize {
        self.cards.iter().filter(|card| card.is_some()).count()
    }

    pub(crate) fn get_cards(&self) -> Vec<&Option<Card>> {
        self.cards.iter().filter(|card| card.is_some()).collect()
    }

    // "compress" the playing field, removing any None spaces
    // this will take effect mostly at the end of the game, when there aren't any cards in the deck
    // it will also take effect when we had more than 12 cards on the field at some point because there were no sets
    fn compress(&mut self) {
        self.cards.retain(|card| card.is_some());
    }

    pub fn add_cards(&mut self, mut added: Vec<Card>) {
        // Add cards, beginning with filling the empty spaces and then appending to the end of the vector
        for card in self.cards.iter_mut() {
            if card.is_none() {
                continue;
            }
            if let Some(new_card) = added.pop() {
                *card = Some(new_card);
            }
        }

        self.compress();

        self.cards.extend(added.into_iter().map(Some));
    }
}
