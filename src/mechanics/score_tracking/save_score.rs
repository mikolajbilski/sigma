use bevy::prelude::{Event, EventReader, NextState, Query, ResMut};

use crate::{
    save::{
        date::get_current_date,
        saving::{load_top_scores, save_stats},
    },
    score_counter::ScoreInfo,
    states::AppState,
    timer::TimerInfo,
};

use super::score::Score;

#[derive(Event)]
pub(crate) struct SaveScoreEvent {}

pub(crate) fn save_score(
    mut ev_save: EventReader<SaveScoreEvent>,
    time_query: Query<&TimerInfo>,
    score_query: Query<&ScoreInfo>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in ev_save.read() {
        let timer = time_query.get_single().unwrap();
        let time = timer.get_time();

        let score_tracker = score_query.get_single().unwrap();
        let score = score_tracker.get_score();

        let date = get_current_date();

        let mut highscores = load_top_scores();

        let new_score = Score::new(time, score, date);

        let rank = highscores.add_score(new_score);

        //TODO: give user some feedback about his score
        println!("Your rank is: {}", rank);
        println!("You have played {} games", highscores.get_played_games());

        if rank > 0 {
            save_stats(&highscores);
        }

        // We do it here because we want to ensure all stats are saved first
        next_state.set(AppState::GameOver);
    }
}
