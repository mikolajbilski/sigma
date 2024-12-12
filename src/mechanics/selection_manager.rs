use std::borrow::BorrowMut;

use bevy::prelude::*;

use crate::card::{self, Card};

use super::set::is_set;

pub fn check_selected(mut card_query: Query<&mut Card>) {
    let mut selected_count: usize = 0;

    for card in &card_query {
        if card.is_selected() {
            selected_count += 1;
        }
    }

    if selected_count == 3 {
        println!("CHECKING FOR A SET...");
        for mut card in &mut card_query {
            if card.is_selected() {
                card.flip_selection();
            }
        }
    }
}
