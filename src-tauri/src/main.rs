// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Thought {
    id: u32,
    title: String,
    relation_id: u32,
    x: u32,
    y: u32,
}

#[tauri::command]
fn get_last_id() -> Result<u32, String> {
    let json_content = include_str!("../src/thoughts.json");
    let data: Vec<serde_json::Value> =
        serde_json::from_str(json_content).map_err(|e| format!("Error parsing JSON: {}", e))?;

    let highest_id = data
        .iter()
        .filter_map(|obj| obj.get("id").and_then(|id| id.as_u64()))
        .max()
        .ok_or_else(|| "No ID found in JSON".to_string())?;

    Ok(highest_id as u32)
}

#[tauri::command]
fn read_json() -> Result<String, String> {
    let json_content = include_str!("../src/thoughts.json");
    let data: Vec<Thought> = serde_json::from_str(json_content)
        .map_err(|e| format!("Error parsing JSON: {}(in fn read_json)", e))?;

    let json_content = serde_json::to_string(&data)
        .map_err(|e| format!("Error converting JSON to String: {} (in fn read_json)", e))?;

    Ok(json_content)
}

#[tauri::command]
fn write_json(data: String) {
    let existing_data = fs::read_to_string("./src/thoughts.json");

    let mut thought: Vec<Thought> = match existing_data {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    };

    if let Ok(data) = serde_json::from_str::<Vec<Thought>>(&data) {
        for new_thought in data {
            if let Some(existing_thought) = thought
                .iter_mut()
                .find(|thought| thought.id == new_thought.id)
            {
                //for updating existing one
                *existing_thought = new_thought;
            } else {
                thought.push(new_thought);
            }
        }
    }

    let serialized_data = serde_json::to_string_pretty(&thought).expect("Failed to serialize data");
    fs::write("./src/thoughts.json", serialized_data).expect("Failed to write file");
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_last_id, write_json, read_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
