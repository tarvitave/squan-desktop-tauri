// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod standalone;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Squan.", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            standalone::standalone_chat,
            standalone::save_settings,
            standalone::load_settings,
            standalone::test_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
