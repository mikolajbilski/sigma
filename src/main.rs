mod card;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_card)
        .run();
}

pub fn spawn_card(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let deck = card::Deck::new();

    let card = deck.peek();

    commands
        .spawn(TransformBundle::from_transform(Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            0.0,
        ))) // Initial card position
        .with_children(card.generate_card_entity(&asset_server));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
