use bevy::{prelude::*, window::PrimaryWindow};

use crate::card::card;

use self::card::Card;

use super::card_clicked_event::CardClickedEvent;

pub(crate) fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut card_query: Query<(Entity, &mut Card, &Transform)>,
    mut ev_selected: EventWriter<CardClickedEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(win) = window_query.get_single() {
            if let Some(pos) = win.cursor_position() {
                // Calculating click_y is required because world has y-axis increasing upwards, and window the opposite
                let click_x = pos[0];
                let click_y = 900.0 - pos[1];

                for (entity, _, transform) in &mut card_query {
                    let card_x = transform.translation[0];
                    let card_y = transform.translation[1];
                    if click_x > card_x - 90.0
                        && click_x < card_x + 90.0
                        && click_y > card_y - 54.0
                        && click_y < card_y + 54.0
                    {
                        ev_selected.send(CardClickedEvent { entity });
                        break;
                    }
                }
            }
        } else {
            panic!("No window found while processing mouse input!");
        }
    }
}
