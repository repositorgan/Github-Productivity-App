use serde::Serialize;

#[derive(Serialize)]
pub struct EconomicReport {
    pub total_sessions: i32,
    pub total_cost: f64,
    pub total_minutes_saved: i32,
    pub value_produced: f64,
    pub roi: f64,
    pub productivity_multiplier: f64,
}

pub fn calculate_economics(
    sessions: i32,
    total_cost: f64,
    commits: i32,
    hourly_rate: f64,
) -> EconomicReport {
    let minutes_saved_per_session = 20;

    let total_minutes_saved = sessions * minutes_saved_per_session;

    let value_produced =
        (total_minutes_saved as f64 / 60.0) * hourly_rate;

    let roi = if total_cost > 0.0 {
        value_produced / total_cost
    } else {
        0.0
    };

    let baseline_commits = commits as f64 * 0.7;

    let productivity_multiplier = if baseline_commits > 0.0 {
        commits as f64 / baseline_commits
    } else {
        1.0
    };

    EconomicReport {
        total_sessions: sessions,
        total_cost,
        total_minutes_saved,
        value_produced,
        roi,
        productivity_multiplier,
    }
}
