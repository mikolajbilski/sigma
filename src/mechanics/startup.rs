use bevy::{prelude::*, window::PrimaryWindow};

use crate::card::card;

use super::{
    card_clicked_event::CardClickedEvent,
    found_set_event,
    game_manager::{self, GameManager},
    input_manager,
    playing_field::{self, remove_found_set},
    score_counter, selection_manager, timer,
};

const DEFAULT_WIDTH: f32 = 1280.0;
const DEFAULT_HEIGHT: f32 = 900.0;

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.get_single() {
        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        });
    } else {
        panic!("Unable to spawn a camera as there is no game window!");
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(GameManager::new());
}

fn start_game(mut game_query: Query<&mut GameManager>) {
    if let Ok(game_manager) = game_query.get_single_mut() {
        game_manager.into_inner().start_game();
    } else {
        panic!("Failed to locate GameManager!");
    }
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

pub(crate) fn init() {
    App::new()
        .add_plugins(DefaultPlugins.set(generate_window()))
        .add_systems(Startup, (spawn_camera, timer::setup, score_counter::setup))
        .add_systems(Startup, (startup, start_game).chain())
        .add_systems(
            Update,
            (
                playing_field::display,
                input_manager::handle_mouse_clicks,
                selection_manager::check_selected,
                remove_found_set,
                playing_field::move_to_compress,
                timer::update_timer,
                timer::stop_timer,
                score_counter::update_score,
                card::flip_selection,
                playing_field::unselect_all_cards,
            ),
        )
        .add_event::<found_set_event::FoundSetEvent>()
        .add_event::<playing_field::MoveCardsEvent>()
        .add_event::<game_manager::GameEndedEvent>()
        .add_event::<CardClickedEvent>()
        .add_event::<playing_field::UnselectAllEvent>()
        .run();
}
