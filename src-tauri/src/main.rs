#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kvstore;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
