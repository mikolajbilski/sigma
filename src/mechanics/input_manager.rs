use bevy::{prelude::*, window::PrimaryWindow};

use crate::card::Card;

pub fn handle_mouse_clicks(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    card_query: Query<&Card, With<Transform>>,
) {
    let win = window_query.get_single().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", win.cursor_position());
        for card in &card_query {
            println!("{:?}", card);
        }
    }
}