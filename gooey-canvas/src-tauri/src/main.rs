use std::env;
mod canvas;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
    println!("{}", get_user_courses());
    format!("Hello from rust! Your name is {}", name)
}

//returns a json string of the user's courses
#[tauri::command]
fn get_user_courses() -> String {
    let joint = &canvas::get_apikey();
    let courses = canvas::get_courses(joint);
    format!("{}", courses)
}

//generate handlers for typescript
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_user_courses])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
