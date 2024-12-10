use std::collections::VecDeque;

use super::Card;

use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Component)]
pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Card::generate_all();

        cards.shuffle(&mut thread_rng());

        Deck { cards: VecDeque::from(cards) }
    }

    pub fn peek(&self) -> Card {
        self.cards.front().unwrap().clone()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    // Return no_cards from the top of the deck, removing them from the deck
    // If there are less cards in the deck than requested, returns all the remaining cards
    pub fn get_cards(&mut self, no_cards: usize) -> Vec<Card> {
        let mut taken = vec![];

        for _ in 0..no_cards {
            if let Some(card) = self.cards.pop_front() {
                taken.push(card);
            } else {
                break;
            }
        }

        taken
    }
}
