// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    github_productivity_app_lib::run()
}

mod economics;

#[tauri::command]
fn run_economic_model(
    sessions: i32,
    total_cost: f64,
    commits: i32,
    hourly_rate: f64,
) -> economics::EconomicReport {
    economics::calculate_economics(
        sessions,
        total_cost,
        commits,
        hourly_rate,
    )
}

.invoke_handler(tauri::generate_handler![
    run_economic_model
])
