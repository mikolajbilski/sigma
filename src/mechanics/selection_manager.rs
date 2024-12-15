use bevy::prelude::*;

use crate::card::Card;

use super::{found_set_event::FoundSetEvent, set::is_set};

pub fn check_selected(
    mut card_query: Query<&mut Card>,
    mut ev_foundset: EventWriter<FoundSetEvent>,
) {
    let mut selected_count: usize = 0;

    for card in &card_query {
        if card.is_selected() {
            selected_count += 1;
        }
    }

    let mut to_check = vec![];

    if selected_count == 3 {
        for card in &mut card_query {
            if card.is_selected() {
                to_check.push(card.clone());
            }
        }
        if is_set(&to_check[0], &to_check[1], &to_check[2]) {
            for mut card in &mut card_query {
                if card.is_selected() {
                    card.mark_for_removal();
                }
            }
            ev_foundset.send(FoundSetEvent::new(to_check));
        } else {
            for mut card in &mut card_query {
                if card.is_selected() {
                    card.flip_selection();
                }
            }
        }
    }
}
