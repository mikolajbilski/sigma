use bevy::prelude::*;

use crate::card::Card;

use super::set::is_set;

#[derive(Component)]
pub struct PlayingField {
    // Represents 3x4 - 3x7 field of cards, None means we want to refill this space
    cards: Vec<Option<Card>>,
}

pub fn display(
    mut commands: Commands,
    field_query: Query<&PlayingField>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(playing_field) = field_query.get_single() {
        let no_cards = playing_field.cards_count();
        match no_cards {
            12 => {}
            _ => {}
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
