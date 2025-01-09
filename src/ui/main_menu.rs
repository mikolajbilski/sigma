use bevy::{color::Color, prelude::*};

use crate::states::AppState;

use super::button_markers::ButtonTypeMarker;

const BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub(crate) struct MenuMarker {}

pub(crate) fn spawn_menu(mut commands: Commands) {
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
        MenuMarker {},
    );

    let start_game_button = generate_button(ButtonTypeMarker::StartGame);
    let stats_button = generate_button(ButtonTypeMarker::DisplayStats);
    let exit_button = generate_button(ButtonTypeMarker::Exit);

    let start_game_text = commands
        .spawn(generate_button_bundle_text("Start Game"))
        .id();
    let stats_text = commands.spawn(generate_button_bundle_text("Stats")).id();
    let exit_text = commands.spawn(generate_button_bundle_text("Exit")).id();

    let container = commands.spawn(container_node).id();
    let start_game_button = commands.spawn(start_game_button).id();
    let stats_button = commands.spawn(stats_button).id();
    let exit_button = commands.spawn(exit_button).id();

    commands
        .entity(start_game_button)
        .push_children(&[start_game_text]);
    commands.entity(stats_button).push_children(&[stats_text]);
    commands.entity(exit_button).push_children(&[exit_text]);
    commands
        .entity(container)
        .push_children(&[start_game_button, stats_button, exit_button]);
}

pub(crate) fn main_menu_system(
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
                ButtonTypeMarker::StartGame => {
                    println!("STARTING A GAME!");
                    next_state.set(AppState::Game);
                }
                ButtonTypeMarker::DisplayStats => {
                    println!("DISPLAYING STATS!");
                    //TODO: Display stats
                }
                ButtonTypeMarker::Exit => {
                    println!("EXITING!");
                    ev_exit.send(AppExit::Success);
                }
            };
        }
    }
}

fn generate_button_bundle_text(content: &str) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 40.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    )
}

fn generate_button(buton_type: ButtonTypeMarker) -> (ButtonBundle, ButtonTypeMarker) {
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

pub(crate) fn destroy_menu(
    mut commands: Commands,
    mut menu_query: Query<Entity, With<MenuMarker>>,
) {
    for menu_item in &mut menu_query {
        commands.entity(menu_item).despawn_recursive();
    }
}
