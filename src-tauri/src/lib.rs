// src-tauri/src/lib.rs
mod economics;
mod github;
mod analytics;
mod db;

use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

// Simple struct to hold credentials in memory for now.
// You can later persist these to a file or keyring if you want.
#[derive(Debug, Clone)]
struct Credentials {
    username: String,
    token: String,
}

// This will live in app state.
struct AppState {
    credentials: std::sync::Mutex<Option<Credentials>>,
}

#[tauri::command]
fn save_credentials(
    app: tauri::AppHandle,
    username: String,
    token: String,
) -> Result<(), String> {
    db::save_credentials(&app, &username, &token)
}

    // Optional: show a confirmation dialog
    if let Some(window) = app.get_webview_window("main") {
        window
          println!("GitHub credentials saved successfully.");
    }
    Ok(())
}

#[tauri::command]
fn get_credentials(app: tauri::AppHandle) -> Result<Credentials, String> {
    db::get_credentials(&app)
        Err("No credentials found".to_string())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_credentials,
            get_credentials,
            github::get_14_day_commits
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

