// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;


#[tauri::command]
fn write_json(data: String) {
    fs::write("./src/toughts.json", data).expect("Failed to write file");
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            write_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
