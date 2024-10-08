// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;

#[derive(Serialize, Deserialize)]

struct Thought {
    id: u32,
    title: String,
    relation_id: u32,
    added_relation_id: u32,
    x: u32,
    y: u32,
}

#[tauri::command]
fn get_last_id() -> Result<u32, String> {
    // Get the application directory path
    let path: PathBuf = [
        app_data_dir(&tauri::Config::default())
            .expect("Failed to get app directory")
            .to_str()
            .unwrap(),
        "thoughts.json",
    ]
    .iter()
    .collect();
    let json_content =
        fs::read_to_string(&path).map_err(|e| format!("Error reading file: {}", e))?;
    //let json_content = include_str!("../src/thoughts.json");

    let data: Vec<serde_json::Value> =
        serde_json::from_str(&json_content).map_err(|e| format!("Error parsing JSON: {}", e))?;

    // Pick highest ID from Thought Structs
    let highest_id = data
        .iter()
        .filter_map(|obj| obj.get("id").and_then(|id| id.as_u64()))
        .max()
        .ok_or_else(|| "No ID found in JSON".to_string())?;

    Ok(highest_id as u32)
}

#[tauri::command]
fn read_json() -> Result<String, String> {
    // Get the application directory path
    let app_dir = app_data_dir(&tauri::Config::default()).expect("Failed to get app directory");
    let path: PathBuf = [app_dir.to_str().unwrap(), "thoughts.json"]
        .iter()
        .collect();
    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create directories");
        }
    }

    // If the file doesn't exist, create it with an empty JSON array
    if !path.exists() {
        std::fs::write(
            &path,
            r#"[{
        "id": 1,
        "title": "Mieli",
        "relation_id": 0,
        "added_relation_id": 0,
        "x": 8000,
        "y": 4500
        }]"#,
        )
        .expect("Failed to create thoughts.json");
        println!("File created{}", path.display());
    }

    //let json_content = include_str!("../src/thoughts.json");
    let json_content = fs::read_to_string(&path).map_err(|e| format!("Error reading file: {}", e));

    let data: Vec<Thought> = serde_json::from_str(&json_content?)
        .map_err(|e| format!("Error parsing JSON: {}(in fn read_json)", e))?;

    let json_content = serde_json::to_string(&data)
        .map_err(|e| format!("Error converting JSON to String: {} (in fn read_json)", e))?;

    Ok(json_content)
}

#[tauri::command]
fn write_json(data: String) {
    // Get the application directory path
    let app_dir = app_data_dir(&tauri::Config::default()).expect("Failed to get app directory");
    let path: PathBuf = [app_dir.to_str().unwrap(), "thoughts.json"]
        .iter()
        .collect();

    // Ensure the directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create directories");
        }
    }

    // If the file doesn't exist, create it with an empty JSON array
    if !path.exists() {
        std::fs::write(
            &path,
            r#"[{
        "id": 1,
        "title": "Mieli",
        "relation_id": 0,
        "added_relation_id": 0,
        "x": 8000,
        "y": 4500
        }]"#,
        )
        .expect("Failed to create thoughts.json");
        println!("File created{}", path.display());
    }

    //let existing_data = fs::read_to_string("./src/thoughts.json");
    let existing_data = fs::read_to_string(&path);

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
    //fs::write("./src/thoughts.json", serialized_data).expect("Failed to write file");
    fs::write(&path, serialized_data).expect("Failed to write file");
}
#[tauri::command]
fn delete_data() {
    // get the application directory path
    let app_dir = app_data_dir(&tauri::Config::default()).expect("Failed to get app directory");
    let path: PathBuf = [app_dir.to_str().unwrap(), "thoughts.json"]
        .iter()
        .collect();
    // delete file if exists and call read function to create new file
    if path.exists() {
        std::fs::remove_file(&path).expect("Failed to delete file");
        read_json().expect("Failed to read file");
    }
    // dirty handling for file not found
    else {
        println!("File not found{}", path.display());
    }
}
#[tauri::command]
fn update_json(targetid: u32, addedrelationid: u32) {
    // get the application directory path
    let app_dir = app_data_dir(&tauri::Config::default()).expect("Failed to get app directory");
    let path: PathBuf = [app_dir.to_str().unwrap(), "thoughts.json"]
        .iter()
        .collect();

    // check if file exists
    if path.exists() {
        // read file and parse JSON
        let data: String = fs::read_to_string(&path).expect("Failed to read file");
        let mut thoughts: Vec<Thought> = serde_json::from_str(&data).expect("Failed to parse JSON");

        // check matching id and update added_relation_id
        if let Some(thought) = thoughts.iter_mut().find(|t| t.id == targetid) {
            thought.added_relation_id = addedrelationid;
        } else {
            println!("Thought with id {} not found", targetid);
        }
        let updated_data =
            serde_json::to_string_pretty(&thoughts).expect("Failed to serialize data");

        fs::write(&path, updated_data).expect("Failed to write file");
    } else {
        println!("File not found{}", path.display());
    }
}
#[tauri::command]
fn delete_entry(targetid: u32) {
    // get the application directory path
    let app_dir = app_data_dir(&tauri::Config::default()).expect("Failed to get app directory");
    let path: PathBuf = [app_dir.to_str().unwrap(), "thoughts.json"]
        .iter()
        .collect();

    // check if file exists
    if path.exists() {
        // read file and parse JSON
        let data: String = fs::read_to_string(&path).expect("Failed to read file");
        let mut thoughts: Vec<Thought> = serde_json::from_str(&data).expect("Failed to parse JSON");

        // check matching id and remove entry
        if let Some(index) = thoughts.iter().position(|t| t.id == targetid) {
            thoughts.remove(index);
        } else {
            println!("Thought with id {} not found", targetid);
        }
        let updated_data =
            serde_json::to_string_pretty(&thoughts).expect("Failed to serialize data");

        fs::write(&path, updated_data).expect("Failed to write file");
    } else {
        println!("File not found{}", path.display());
    }
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_last_id,
            write_json,
            read_json,
            delete_data,
            update_json,
            delete_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
