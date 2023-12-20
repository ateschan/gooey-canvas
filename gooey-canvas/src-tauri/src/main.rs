mod canvas;
mod mock;
mod tests;
use canvas::apikey::{set_apikey, validate_key};
use canvas::canvas::{canvas_get_assignments, canvas_get_courses, canvas_get_user};
use std::env;

//This file contains the backend endpoints.

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(key: &str) -> String {
    if validate_key(&key) == false {
        if key == "" {
            return "No API Key".to_string();
        }
        return "Invalid API Key".to_string();
    } else {
        println!("Valid API Key");
        set_apikey(key);
        //tests::test_canvas_functions();
        let user = canvas_get_user();
        let name: &str = &user.name;
        return format!("Hello {}", name);
    }
}

#[tauri::command]
fn get_user_courses_assignment(id: i32) -> serde_json::Value {
    let assignments = canvas_get_assignments(id);
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&assignments).unwrap()).unwrap();
    return json;
}

#[tauri::command]
fn get_user_courses() -> serde_json::Value {
    let user = &canvas_get_user();
    let courses = canvas_get_courses(user.id);
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&courses).unwrap()).unwrap();
    return json;
}

#[tauri::command]
fn get_user() -> serde_json::Value {
    let user = &canvas_get_user();
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap();
    return json;
}

#[tauri::command]
fn mock_greet(key: &str) -> String {
    if validate_key(&key) == false {
        if key == "" {
            return "No API Key".to_string();
        }
        return "Invalid API Key".to_string();
    } else {
        println!("Valid API Key");
        set_apikey(key);
        //tests::test_canvas_functions();
        let user = mock::get_user();
        let name: &str = &user.name;
        return format!("(MOCK) Hello {}", name);
    }
}

#[tauri::command]
fn mock_get_user_courses_assignment(course_id: i32) -> serde_json::Value {
    let assignments = mock::return_mockedassignments(course_id);
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&assignments).unwrap()).unwrap();
    return json;
}

#[tauri::command]
fn mock_get_user_courses() -> serde_json::Value {
    let user = mock::get_user();
    let courses = mock::return_mockedcourses(user.id);
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&courses).unwrap()).unwrap();
    return json;
}

#[tauri::command]
fn mock_get_user() -> serde_json::Value {
    let user = mock::get_user();
    let json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&user).unwrap()).unwrap();
    return json;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user_courses_assignment,
            get_user_courses,
            get_user,
            mock_greet,
            mock_get_user_courses_assignment,
            mock_get_user_courses,
            mock_get_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
