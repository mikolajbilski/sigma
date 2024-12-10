use bevy::prelude::*;

use crate::card::Card;

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
            12 => {},
            _ => {},
        }
    }
}

impl PlayingField {
    pub fn new() -> Self {
        PlayingField {
            cards: vec![],
        }
    }

    pub fn contains_set(&self) -> bool {
        //TODO: stub
        true
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