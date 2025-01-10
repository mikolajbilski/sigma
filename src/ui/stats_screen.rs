use bevy::prelude::*;

use crate::states::AppState;

use super::{
    button_markers::ButtonTypeMarker,
    buttons::{generate_button, generate_button_bundle_text},
};

#[derive(Component)]
pub(crate) struct StatsMarker {}

pub(crate) fn spawn_stats(mut commands: Commands) {
    let container_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        StatsMarker {},
    );

    let start_game_button = generate_button(ButtonTypeMarker::MainMenu);

    let start_game_text = commands.spawn(generate_button_bundle_text("Go back")).id();

    let container = commands.spawn(container_node).id();
    let start_game_button = commands.spawn(start_game_button).id();

    commands
        .entity(start_game_button)
        .push_children(&[start_game_text]);
    commands
        .entity(container)
        .push_children(&[start_game_button]);
}

pub(crate) fn stats_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonTypeMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                ButtonTypeMarker::MainMenu => {
                    next_state.set(AppState::Menu);
                }
                _ => panic!("This button shouldn't be here!"),
            };
        }
    }
}

pub(crate) fn destroy_stats(
    mut commands: Commands,
    mut menu_query: Query<Entity, With<StatsMarker>>,
) {
    for menu_item in &mut menu_query {
        commands.entity(menu_item).despawn_recursive();
    }
}
