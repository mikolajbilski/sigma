use bevy::prelude::Event;

use crate::card::Card;

#[derive(Event)]
pub struct FoundSetEvent {
    cards: Vec<Card>,
}

impl FoundSetEvent {
    pub fn new(cards: Vec<Card>) -> Self {
        FoundSetEvent { cards }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
}
