use bevy::prelude::*;

use super::found_set_event::FoundSetEvent;

#[derive(Component)]
pub(crate) struct ScoreInfo {
    score: u32,
}

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(800.0, 800.0, 0.0),
            ..Default::default()
        })
        .insert(ScoreInfo { score: 0 });
}

pub(crate) fn update_score(
    mut query: Query<(&mut Text, &mut ScoreInfo)>,
    mut found_set_event: EventReader<FoundSetEvent>,
) {
    for _ in found_set_event.read() {
        for (mut text, mut score_info) in query.iter_mut() {
            let new_score = score_info.score + 1;
            text.sections[0].value = format!("Score: {}", new_score);
            score_info.score = new_score;
        }
    }
}
