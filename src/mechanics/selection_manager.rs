use bevy::prelude::*;

use crate::card::card;

use self::card::Card;

use super::{card_clicked_event::CardClickedEvent, found_set_event::FoundSetEvent, playing_field::UnselectAllEvent, set::is_set};



pub(crate) fn check_selected(
    mut card_query: Query<&mut Card>,
    mut ev_foundset: EventWriter<FoundSetEvent>,
    mut card_selected_event: EventReader<CardClickedEvent>,
    mut unselect_all_event: EventWriter<UnselectAllEvent>,
) {
    for _ in card_selected_event.read() {
        let mut selected_count: usize = 0;

        for card in &card_query {
            if card.is_selected() {
                selected_count += 1;
            }
        }

        if selected_count >= 3 {
            let mut to_check = vec![];

            for card in &mut card_query {
                if card.is_selected() {
                    to_check.push(*card);
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
                println!("NOT A SET");
                unselect_all_event.send(UnselectAllEvent {});
            }
        }
    }
}
