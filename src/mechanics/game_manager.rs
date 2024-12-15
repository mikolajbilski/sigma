// Manages a single game, including the deck, playing field, scores and timer

use std::cmp::max;

use bevy::prelude::Component;

use crate::card::{Card, Deck};

use super::playing_field::PlayingField;

#[derive(Component)]
pub struct GameManager {
    playing_field: PlayingField,
    deck: Deck,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            playing_field: PlayingField::new(),
            deck: Deck::new(),
        }
    }

    // Fill the playing field until there are at least 12 cards and there is at least one set
    pub fn fill_playing_field(&mut self) {
        let cards_to_add = max(12 - self.playing_field.cards_count(), 0);
        println!("FILLING THE FIELD WITH {} CARDS!", cards_to_add);
        let added_cards = self.deck.take_cards(cards_to_add);
        self.playing_field.add_cards(added_cards);
        while !(self.deck.is_empty() || self.playing_field.contains_set()) {
            // Always add cards in increment of 3
            self.playing_field.add_cards(self.deck.take_cards(3));
        }
    }

    pub fn remove_cards(&mut self, cards: Vec<Card>) {
        self.playing_field.remove_cards(cards);
    }

    pub fn start_game(&mut self) {
        // Init the playing field for 12 starting cards
        println!("GAME BEGINS");
        self.fill_playing_field();
    }

    pub fn get_playing_field(&self) -> &PlayingField {
        &self.playing_field
    }

    pub fn get_playing_field_mut(&mut self) -> &mut PlayingField {
        &mut self.playing_field
    }
}
