use chrono::prelude::*;

pub(crate) fn get_current_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
