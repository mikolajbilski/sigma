use super::Card;

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Component)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Card::generate_all();

        cards.shuffle(&mut thread_rng());

        Deck { cards }
    }

    pub fn peek(&self) -> Card {
        self.cards.first().unwrap().clone()
    }
}
