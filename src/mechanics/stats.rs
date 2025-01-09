use serde::{Deserialize, Serialize};

use super::score::Score;

const SCORES_TRACKED: usize = 10;

#[derive(Serialize, Deserialize)]
pub(crate) struct Stats {
    games_played: u32,
    highscores: Vec<Score>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            games_played: 0,
            highscores: vec![],
        }
    }

    fn add_score(&mut self, new_score: Score) {
        self.games_played += 1;

        // Sorting every time is fine here as there are only a few best scores tracked

        self.highscores.push(new_score);

        self.highscores.sort();

        if self.highscores.len() > SCORES_TRACKED {
            self.highscores.pop();
        }
    }
}