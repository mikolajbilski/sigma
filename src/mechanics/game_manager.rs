// Manages a single game, including the deck, playing field and scores

use std::cmp::max;

use bevy::prelude::{Component, Event};

use crate::card::Deck;

use super::playing_field::PlayingField;

#[derive(Component)]
pub(crate) struct GameManager {
    playing_field: PlayingField,
    deck: Deck,
}

#[derive(Event)]
pub(crate) struct GameEndedEvent {}

impl GameManager {
    pub(crate) fn new() -> Self {
        GameManager {
            playing_field: PlayingField::new(),
            deck: Deck::new(),
        }
    }

    // Fill the playing field until there are at least 12 cards and there is at least one set
    // Returns true if the game has ended
    pub(crate) fn fill_playing_field(&mut self) -> bool {
        let cards_to_add = 12usize.saturating_sub(self.playing_field.cards_count());
        let added_cards = self.deck.take_cards(cards_to_add);
        self.playing_field.add_cards(added_cards);

        while !(self.deck.is_empty() || self.playing_field.contains_set()) {
            // Always add cards in increment of 3
            self.playing_field.add_cards(self.deck.take_cards(3));
        }

        if self.deck.is_empty() && !self.playing_field.contains_set() {
            return true;
        }
        false
    }

    pub(crate) fn start_game(&mut self) {
        println!("GAME BEGINS");
        self.fill_playing_field();
    }

    pub(crate) fn get_playing_field_mut(&mut self) -> &mut PlayingField {
        &mut self.playing_field
    }
}
