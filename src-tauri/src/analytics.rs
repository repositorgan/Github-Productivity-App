use chrono::{Duration, Utc};

pub fn last_14_days() -> Vec<String> {
    let mut days = vec![];

    for i in 0..14 {
        let day = Utc::now() - Duration::days(i);
        days.push(day.format("%Y-%m-%d").to_string());
    }

    days
}

