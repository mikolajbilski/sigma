use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    card::card,
    ui::{
        game_over::{destroy_game_over_screen, game_over_system, spawn_game_over_screen},
        main_menu::{destroy_menu, main_menu_system, spawn_menu},
        stats_screen::{destroy_stats, spawn_stats, stats_system},
    },
};

use self::selection_manager::CheckSelectedEvent;

use super::{
    card_clicked_event::CardClickedEvent,
    found_set_event,
    game_cleanup::game_cleanup,
    game_manager::{self, GameManager},
    input_manager,
    playing_field::{self, remove_found_set},
    score_counter,
    score_tracking::save_score::{save_score, SaveScoreEvent},
    selection_manager,
    states::AppState,
    timer,
};

const DEFAULT_WIDTH: f32 = 1280.0;
const DEFAULT_HEIGHT: f32 = 900.0;

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = window_query.get_single() else {
        panic!("Unable to spawn a camera as there is no game window!");
    };
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
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
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(AppState::Game),
            (timer::setup, score_counter::setup),
        )
        .add_systems(OnEnter(AppState::Game), (startup, start_game).chain())
        .add_systems(OnEnter(AppState::Menu), spawn_menu)
        .add_systems(OnEnter(AppState::GameOver), spawn_game_over_screen)
        .add_systems(OnEnter(AppState::Stats), spawn_stats)
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
                save_score,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(Update, main_menu_system.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), destroy_menu)
        .add_systems(
            Update,
            game_over_system.run_if(in_state(AppState::GameOver)),
        )
        .add_systems(OnExit(AppState::GameOver), destroy_game_over_screen)
        .add_systems(OnExit(AppState::Game), game_cleanup)
        .add_systems(Update, stats_system.run_if(in_state(AppState::Stats)))
        .add_systems(OnExit(AppState::Stats), destroy_stats)
        .add_event::<found_set_event::FoundSetEvent>()
        .add_event::<playing_field::MoveCardsEvent>()
        .add_event::<game_manager::GameEndedEvent>()
        .add_event::<CardClickedEvent>()
        .add_event::<playing_field::UnselectAllEvent>()
        .add_event::<CheckSelectedEvent>()
        .add_event::<SaveScoreEvent>()
        .run();
}
