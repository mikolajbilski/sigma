
// Manages a single game, including the deck, playing field, scores and timer

use crate::card::Deck;

use super::playing_field::PlayingField;

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

    pub fn start_game(&mut self) {
        
    }
}