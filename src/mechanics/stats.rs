use serde::{Deserialize, Serialize};

use super::score::Score;

const SCORES_TRACKED: usize = 10;

#[derive(Serialize, Deserialize)]
pub(crate) struct Stats {
    games_played: u32,
    highscores: Vec<Score>,
}

impl Stats {
    pub(crate) fn new() -> Self {
        Stats {
            games_played: 0,
            highscores: vec![],
        }
    }

    // Returns the position at which the new score is on the highscore list
    // It is 1-indexed (so new best score returns 1)
    // If the new score doesn't make it to the highscore list, returns 0
    pub(crate) fn add_score(&mut self, new_score: Score) -> usize {
        self.games_played += 1;

        // Sorting every time is fine here as there are only a few best scores tracked

        self.highscores.push(new_score.clone());

        self.highscores.sort_by(|a, b| b.cmp(a));

        if self.highscores.len() > SCORES_TRACKED {
            self.highscores.pop();
        }

        if let Some(index) = self.highscores.iter().position(|x| *x == new_score) {
            index + 1
        } else {
            0
        }
    }
}
