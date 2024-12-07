mod card;

use bevy::math::vec3;
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

    //let mut sprite: Handle<Image> = asset_server.load("sprites/diamond.png");

    let deck = card::Deck::new();



    let card = deck.peek();

    let col = card.get_texture_color();


    /* let background_handle = asset_server.load("sprites/general/card_background.png");
    let content_handle = asset_server.load(card.get_asset_path()); */

    commands.spawn(TransformBundle::from_transform(Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0))) // Initial card position
    .with_children(card.generate_card_entity(&asset_server));

    /* commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: background_handle.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Spawn the content sprite as a child
            parent.spawn(SpriteBundle {
                texture: content_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                sprite: Sprite {
                    color: col,
                    ..default()
                },
                ..Default::default()
            });
        }); */

    /* let mut spb = SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        texture: asset_server.load(card.get_asset_path()),
        ..default()
    };

    spb.sprite.color = col;

    commands.spawn((
        spb,
        card::Deck::new(),
    )); */
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}