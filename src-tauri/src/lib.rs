// src-tauri/src/lib.rs
mod economics;
mod github;
mod analytics;
mod db;

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
async fn save_credentials(
    app: AppHandle,
    username: String,
    token: String,
) -> Result<(), String> {
    // Store in state
    let state = app.state::<AppState>();
    {
        let mut creds = state
            .credentials
            .lock()
            .map_err(|_| "Failed to lock credentials state".to_string())?;
        *creds = Some(Credentials { username, token });
    }

    // Optional: show a confirmation dialog
    if let Some(window) = app.get_webview_window("main") {
        window
            .dialog()
            .message("GitHub credentials saved successfully.")
            .title("Success")
            .show()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn get_credentials(app: AppHandle) -> Result<Option<Credentials>, String> {
    let state = app.state::<AppState>();
    let creds = state
        .credentials
        .lock()
        .map_err(|_| "Failed to lock credentials state".to_string())?;
    Ok(creds.clone())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            credentials: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_credentials,
            get_credentials
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
