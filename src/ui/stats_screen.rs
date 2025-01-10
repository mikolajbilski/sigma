use bevy::prelude::*;

use crate::{save::saving::load_top_scores, score_tracking::stats::Stats, states::AppState};

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

    let stats = load_top_scores();

    let games_played_info = generate_games_played_text(&stats);

    let stats_table = create_stats_table(&mut commands, &stats);

    let go_back_button = generate_button(ButtonTypeMarker::MainMenu);

    let go_back_text = commands
        .spawn(generate_button_bundle_text("Main Menu"))
        .id();

    let container = commands.spawn(container_node).id();
    let games_played_info = commands.spawn(games_played_info).id();
    let go_back_button = commands.spawn(go_back_button).id();

    commands
        .entity(go_back_button)
        .push_children(&[go_back_text]);
    commands
        .entity(container)
        .push_children(&[games_played_info, stats_table, go_back_button]);
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

fn create_stats_table(commands: &mut Commands, stats: &Stats) -> Entity {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(80.0),
                        justify_content: JustifyContent::SpaceBetween,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header| {
                    header.spawn(generate_table_text("Rank"));
                    header.spawn(generate_table_text("Time"));
                    header.spawn(generate_table_text("Sets"));
                    header.spawn(generate_table_text("Date"));
                });

            for (rank, score) in stats.get_highscores().iter().enumerate() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(80.0),
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn(generate_table_text(&(rank + 1).to_string()));
                        row.spawn(generate_table_text(&score.time_as_string()));
                        row.spawn(generate_table_text(&score.sets.to_string()));
                        row.spawn(generate_table_text(&score.date));
                    });
            }
        })
        .id()
}

fn generate_table_text(content: &str) -> TextBundle {
    TextBundle::from_section(
        content,
        TextStyle {
            font_size: 30.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    )
}

fn generate_games_played_text(stats: &Stats) -> TextBundle {
    TextBundle::from_section(
        format!("You have played {} games", stats.get_played_games()),
        TextStyle {
            font_size: 40.0,
            color: Color::srgb(0.9, 0.9, 0.9),
            ..default()
        },
    )
}
