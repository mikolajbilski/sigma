use std::{cmp::Ordering, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub(crate) struct Score {
    time: u128,
    sets: u32,
    date: String,
}

impl Score {
    pub(crate) fn new(time: Duration, sets: u32, date: String) -> Self {
        let time = time.as_millis();
        Score { time, sets, date }
    }

    pub(crate) fn seconds_per_set(&self) -> f32 {
        (self.time as f32 / self.sets as f32) / 1000.0
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_score = self.seconds_per_set();
        let other_score = other.seconds_per_set();

        other_score
            .partial_cmp(&self_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.date.cmp(&other.date))
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
