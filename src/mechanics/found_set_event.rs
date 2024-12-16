use bevy::prelude::Event;

use crate::card::Card;

#[derive(Event)]
pub(crate) struct FoundSetEvent {
    cards: Vec<Card>,
}

impl FoundSetEvent {
    pub(crate) fn new(cards: Vec<Card>) -> Self {
        FoundSetEvent { cards }
    }

    pub(crate) fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }
}
