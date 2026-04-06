// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod standalone;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Squan.", name)
}

#[tauri::command]
async fn standalone_chat(request: standalone::ChatRequest) -> Result<standalone::ChatResponse, String> {
    standalone::chat(request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings(settings: standalone::StandaloneSettings) -> Result<(), String> {
    settings.save().map_err(|e| e.to_string())
}

#[tauri::command]
fn load_settings() -> Result<standalone::StandaloneSettings, String> {
    Ok(standalone::StandaloneSettings::load())
}

#[tauri::command]
async fn test_connection(url: String) -> Result<bool, String> {
    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(response) => Ok(response.status().is_success()),
        Err(_) => Ok(false),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            standalone_chat,
            save_settings,
            load_settings,
            test_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
