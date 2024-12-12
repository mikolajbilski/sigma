use bevy::{prelude::*, window::PrimaryWindow};

use crate::card::Card;

use super::{
    found_set_event,
    game_manager::GameManager,
    input_manager,
    playing_field::{self, remove_found_set},
    selection_manager,
};

const DEFAULT_WIDTH: f32 = 1280.0;
const DEFAULT_HEIGHT: f32 = 900.0;

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    println!("WIDTH: {}, HEIGHT: {}", window.width(), window.height());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn startup(mut commands: Commands) {
    commands.spawn(GameManager::new());
}

fn start_game(mut game_query: Query<&mut GameManager>) {
    let game_manager = game_query.get_single_mut().unwrap().into_inner();
    game_manager.start_game();
}

fn generate_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: (DEFAULT_WIDTH, DEFAULT_HEIGHT).into(),
            title: "SIGMA".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    }
}

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins.set(generate_window()))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Startup,
            (startup, start_game, playing_field::display, list_cards).chain(),
        )
        .add_systems(Update, input_manager::handle_mouse_clicks)
        .add_systems(Update, selection_manager::check_selected)
        .add_systems(Update, remove_found_set)
        .add_event::<found_set_event::FoundSetEvent>()
        .run();
}

//TODO: temp
fn list_cards(cards: Query<&Card>) {
    for card in &cards {
        println!("{:?}", card);
    }
}
