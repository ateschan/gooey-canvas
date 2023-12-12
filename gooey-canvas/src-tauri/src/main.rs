use std::env;
mod canvas;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//set api key
fn set_apikey(key: &str) {
    env::set_var("CANVAS_API_KEY", key);
}

//greet user for api key
#[tauri::command]
fn greet(key: &str) -> String {
    if canvas::validate_key(key) == true {
        return "Invalid API Key".to_string();
    } else if key == "" {
        return "No API Key".to_string();
    } else {
        println!("Valid API Key");
    }
    let user = canvas::get_user(key);
    let name: &str = &user.name;
    set_apikey(key);
    format!("Hello from rust! Your name is {}", name)
}

#[tauri::command]
fn get_course_assignments(id: i32) -> serde_json::Value {
    let key = &canvas::get_apikey();
    let assignments = canvas::retrieve_assignments(key, id);
    let json = serde_json::to_string(&assignments).unwrap();
    return serde_json::from_str(&json).unwrap();
}

//returns a json string of the user's courses
//json is bare right now TODO make a struct for it and simplfy this redundant code
#[tauri::command]
fn get_user_courses() -> serde_json::Value {
    let joint = &canvas::get_apikey();
    let courses = canvas::get_courses(joint);
    let json: serde_json::Value = serde_json::from_str(&courses).unwrap();
    json //return json string
}

#[tauri::command]
fn get_user() -> serde_json::Value {
    let key = &canvas::get_apikey();
    let user = &canvas::get_user(key);
    let json = serde_json::to_string(&user).unwrap();
    return serde_json::from_str(&json).unwrap();
}

//generate handlers for typescript
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_user_courses])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
