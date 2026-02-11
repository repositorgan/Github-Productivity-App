use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Deserialize)]
pub struct Event {
    pub r#type: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct DailyCommit {
    pub date: String,
    pub commits: i32,
}

#[tauri::command]
pub async fn get_14_day_commits(username: String) -> Result<Vec<DailyCommit>, String> {
    let url = format!("https://api.github.com/users/{}/events", username);

    let client = reqwest::Client::new();

    let events: Vec<Event> = client
        .get(url)
        .header("User-Agent", "ai-productivity-app")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let mut days = vec![];

    for i in 0..14 {
        let day = Utc::now() - Duration::days(i);
        days.push((
            day.format("%Y-%m-%d").to_string(),
            0
        ));
    }

    for event in events {
        if event.r#type == "PushEvent" {
            let event_date = DateTime::parse_from_rfc3339(&event.created_at)
                .map_err(|e| e.to_string())?
                .with_timezone(&Utc)
                .format("%Y-%m-%d")
                .to_string();

            for day in days.iter_mut() {
                if day.0 == event_date {
                    day.1 += 1;
                }
            }
        }
    }

    Ok(days.into_iter()
        .map(|(date, commits)| DailyCommit { date, commits })
        .collect())
}

