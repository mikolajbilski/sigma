use bevy::{prelude::*, window::PrimaryWindow};

use crate::card::card;

use self::card::Card;

pub(crate) fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut card_query: Query<(&mut Card, &Transform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(win) = window_query.get_single() {
            if let Some(pos) = win.cursor_position() {
                // Calculating click_y is required because world has y-axis increasing upwards, and window the opposite
                let click_x = pos[0];
                let click_y = 900.0 - pos[1];

                for mut card in &mut card_query {
                    let card_x = card.1.translation[0];
                    let card_y = card.1.translation[1];
                    if click_x > card_x - 90.0
                        && click_x < card_x + 90.0
                        && click_y > card_y - 54.0
                        && click_y < card_y + 54.0
                    {
                        // TODO: highlight the card
                        card.0.flip_selection();
                    }
                }
            }
        } else {
            panic!("No window found while processing mouse input!");
        }
    }
}
