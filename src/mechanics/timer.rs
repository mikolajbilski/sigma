use bevy::prelude::*;
use std::time::Duration;

use super::game_manager::GameEndedEvent;

#[derive(Component)]
pub(crate) struct TimerInfo {
    running: bool,
    time: Duration,
}

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "00.00.00.00s",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(200.0, 800.0, 0.0),
            ..Default::default()
        })
        .insert(TimerInfo { running: true, time: Duration::new(0, 0) });
}

pub(crate) fn update_timer(mut query: Query<(&mut Text, &mut TimerInfo)>, time: Res<Time>) {
    let elapsed_time = time.elapsed();

    let hours = elapsed_time.as_secs() / 3600;
    let minutes = (elapsed_time.as_secs() % 3600) / 60;
    let seconds = elapsed_time.as_secs() % 60;
    let hundredths = (elapsed_time.as_millis() % 1000) / 10;

    for (mut text, mut timer) in query.iter_mut() {
        if timer.running {
            text.sections[0].value = format!(
                "{:02}:{:02}:{:02}.{:02}",
                hours, minutes, seconds, hundredths
            );
            timer.time = elapsed_time;
        }
    }
}

pub(crate) fn stop_timer(
    mut query: Query<(&mut Text, &mut TimerInfo)>,
    mut ev_game_ended: EventReader<GameEndedEvent>,
) {
    for _ in ev_game_ended.read() {
        for (_, mut timer) in query.iter_mut() {
            timer.running = false;
        }
    }
}
