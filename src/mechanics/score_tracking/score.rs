use std::{cmp::Ordering, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub(crate) struct Score {
    pub(crate) time: u128,
    pub(crate) sets: u32,
    pub(crate) date: String,
}

impl Score {
    pub(crate) fn new(time: Duration, sets: u32, date: String) -> Self {
        let time = time.as_millis();
        Score { time, sets, date }
    }

    pub(crate) fn seconds_per_set(&self) -> f32 {
        (self.time as f32 / self.sets as f32) / 1000.0
    }

    pub(crate) fn time_as_string(&self) -> String {
        let total_seconds = self.time / 1000;

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let hundredths = (self.time % 1000) / 10;

        format!(
            "{:02}:{:02}:{:02}.{:02}",
            hours, minutes, seconds, hundredths
        )
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
