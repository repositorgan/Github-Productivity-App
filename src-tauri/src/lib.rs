use std::fs;
use std::process::Command;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
async fn ensure_config(app_handle: AppHandle) -> Result<(), String> {
    let config_path = app_handle
        .path()
        .app_config_dir()
        .unwrap()
        .join("config.json");

    if !config_path.exists() {
        let window = app_handle.get_webview_window("main").unwrap();

        let username = window
            .dialog()
            .input("GitHub Username", "Enter your GitHub username:")
            .await
            .ok_or("Username input cancelled")?;

        let token = window
            .dialog()
            .input("GitHub Token", "Enter your GitHub Personal Access Token:")
            .await
            .ok_or("Token input cancelled")?;

        let json = format!(
            r#"{{"username": "{}", "token": "{}"}}"#,
            username, token
        );

        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        fs::write(&config_path, json).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn get_profile_data(app_handle: AppHandle) -> Result<String, String> {
    let config_path = app_handle
        .path()
        .app_config_dir()
        .unwrap()
        .join("config.json");

    let config: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&config_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

    let token = config["token"]
        .as_str()
        .ok_or("Missing token in config.json")?;

    let script_path = app_handle
        .path()
        .resource_dir()
        .unwrap()
        .join("python")
        .join("fetch_github_profile_stats.py");

    let output = Command::new("python")
        .arg(script_path)
        .env("GITHUB_TOKEN", token)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Python script failed".into());
    }

    let json_path = app_handle
        .path()
        .resource_dir()
        .unwrap()
        .join("profile_productivity.json");

    let json = fs::read_to_string(json_path).map_err(|e| e.to_string())?;
    Ok(json)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![ensure_config, get_profile_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
