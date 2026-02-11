use serde::Deserialize;

#[derive(Deserialize)]
pub struct Event {
    pub r#type: String,
    pub created_at: String,
}

pub async fn fetch_events(username: &str) -> Result<Vec<Event>, reqwest::Error> {
    let url = format!("https://api.github.com/users/{}/events", username);

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("User-Agent", "ai-productivity-app")
        .send()
        .await?
        .json::<Vec<Event>>()
        .await?;

    Ok(res)
}

