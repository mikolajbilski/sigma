use bevy::prelude::*;

use crate::{score_counter::ScoreInfo, states::AppState, timer::TimerInfo};

use super::{
    button_markers::ButtonTypeMarker,
    buttons::{generate_button, generate_button_bundle_text},
};

#[derive(Component)]
pub(crate) struct GameOverMarker {}

pub(crate) fn spawn_game_over_screen(mut commands: Commands) {
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
        GameOverMarker {},
    );

    let game_over_text = TextBundle::from_section(
        "Game finished!",
        TextStyle {
            font_size: 60.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let game_over_text = commands.spawn(game_over_text).id();

    let main_menu_button = generate_button(ButtonTypeMarker::MainMenu);
    let stats_button = generate_button(ButtonTypeMarker::DisplayStats);
    let exit_button = generate_button(ButtonTypeMarker::Exit);

    let main_menu_text = commands
        .spawn(generate_button_bundle_text("Go to Menu"))
        .id();
    let stats_text = commands.spawn(generate_button_bundle_text("Stats")).id();
    let exit_text = commands.spawn(generate_button_bundle_text("Exit")).id();

    let container = commands.spawn(container_node).id();
    let main_menu_button = commands.spawn(main_menu_button).id();
    let stats_button = commands.spawn(stats_button).id();
    let exit_button = commands.spawn(exit_button).id();

    commands
        .entity(main_menu_button)
        .push_children(&[main_menu_text]);
    commands.entity(stats_button).push_children(&[stats_text]);
    commands.entity(exit_button).push_children(&[exit_text]);
    commands.entity(container).push_children(&[
        game_over_text,
        main_menu_button,
        stats_button,
        exit_button,
    ]);
}

pub(crate) fn game_over_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonTypeMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                ButtonTypeMarker::MainMenu => {
                    next_state.set(AppState::Menu);
                }
                ButtonTypeMarker::DisplayStats => {
                    next_state.set(AppState::Stats);
                }
                ButtonTypeMarker::Exit => {
                    ev_exit.send(AppExit::Success);
                }
                _ => panic!("This button shouldn't be here!"),
            };
        }
    }
}

pub(crate) fn destroy_game_over_screen(
    mut commands: Commands,
    mut menu_query: Query<Entity, With<GameOverMarker>>,
    mut timer_query: Query<Entity, With<TimerInfo>>,
    mut score_query: Query<Entity, With<ScoreInfo>>,
) {
    for menu_item in &mut menu_query {
        commands.entity(menu_item).despawn_recursive();
    }

    for timer in &mut timer_query {
        commands.entity(timer).despawn_recursive();
    }

    for score in &mut score_query {
        commands.entity(score).despawn_recursive();
    }
}
