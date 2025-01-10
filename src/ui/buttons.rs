use bevy::prelude::*;

use super::button_markers::ButtonTypeMarker;

const BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub(crate) fn generate_button_bundle_text(content: &str) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 40.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    )
}

pub(crate) fn generate_button(buton_type: ButtonTypeMarker) -> (ButtonBundle, ButtonTypeMarker) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: BUTTON_COLOR.into(),
            ..default()
        },
        buton_type,
    )
}