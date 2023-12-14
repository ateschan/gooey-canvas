use std::env;
mod canvas;
mod tests;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//CONTRIBUTING
//All endpoints must return json, no funny buisness

fn set_apikey(key: &str) {
    env::set_var("CANVAS_API_KEY", key);
}

//greet user for api key
#[tauri::command]
fn greet(key: &str) -> String {
    if canvas::validate_key(&key) == false {
        if key == "" {
            return "No API Key".to_string();
        }
        return "Invalid API Key".to_string();
    } else {
        println!("Valid API Key");
        set_apikey(key);
        let user = canvas::get_user();
        let name: &str = &user.name;
        return format!("Hello from rust! Your name is {}", name);
    }
}

#[tauri::command]
fn get_user_courses_assignment(id: i32) -> serde_json::Value {
    let assignments = canvas::get_assignments(id);
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&assignments).unwrap()).unwrap(); // Returns json
    return json;
}

#[tauri::command]
fn get_user_courses() -> serde_json::Value {
    let user = canvas::get_user();
    let courses = canvas::get_courses(user.id);
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&courses).unwrap()).unwrap(); // Returns json
    return json //return json string
}

#[tauri::command]
fn get_user() -> serde_json::Value {
    let user = &canvas::get_user();
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap(); // Returns json
    return json;
}

//generate handlers for typescript
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user_courses_assignment,
            get_user_courses,
            get_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
