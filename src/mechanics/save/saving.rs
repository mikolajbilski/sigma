use std::fs::{File, OpenOptions};

use directories::ProjectDirs;

use crate::score_tracking::stats::Stats;

fn get_save_file_path() -> String {
    let proj_dirs = ProjectDirs::from("com", "mimuw_students", "sigma_game")
        .expect("Failed to get project directories!");

    let save_dir = proj_dirs.data_local_dir();

    if !save_dir.exists() {
        std::fs::create_dir_all(save_dir)
            .unwrap_or_else(|e| panic!("Failed to create save directory: {}", e));
    }

    save_dir.join("stats.json").to_str().unwrap().to_string()
}

pub(crate) fn save_stats(stats: &Stats) {
    let file_path = get_save_file_path();
    let file =
        File::create(&file_path).unwrap_or_else(|e| panic!("Failed to create savefile: {}", e));
    serde_json::to_writer(file, &stats)
        .unwrap_or_else(|e| panic!("Failed to write to savefile: {}", e));
}

// If the savefile doesn't exist or deserializing it fails, return empty scores
pub(crate) fn load_top_scores() -> Stats {
    let file_path = get_save_file_path();

    match OpenOptions::new().read(true).open(&file_path) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(scores) => scores,
            Err(_) => Stats::new(),
        },
        Err(_) => Stats::new(),
    }
}
