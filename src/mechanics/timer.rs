use bevy::prelude::*;

#[derive(Component)]
pub struct TimerText;

// Setup the UI
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the timer text
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "0.0s",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(200.0, 800.0, 0.0),
            ..Default::default()
        })
        .insert(TimerText);
}

pub fn update_timer(mut query: Query<&mut Text, With<TimerText>>, time: Res<Time>) {
    let elapsed_time = time.elapsed();

    let hours = elapsed_time.as_secs() / 3600;
    let minutes = (elapsed_time.as_secs() % 3600) / 60;
    let seconds = elapsed_time.as_secs() % 60;
    let hundredths = (elapsed_time.as_millis() % 1000) / 10;

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "{:02}:{:02}:{:02}.{:02}",
            hours, minutes, seconds, hundredths
        );
    }
}
