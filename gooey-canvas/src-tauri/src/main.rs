use reqwest;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct User {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "name")]
    name: String,
}

fn get_user(token: &str) -> User {
    let mut user = User {
        id: 0,
        name: "".to_string(),
    };
    match get_data(
        &token,
        "https://alamo.instructure.com/api/v1/users/self/profile",
    ) {
        Ok(body) => {
            user = serde_json::from_str(&body).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    user
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(key: &str) -> String {
    let user = get_user(key);
    let name: &str = &user.name;
    format!("Hello from rust! Your name is {}", name)
}

//Retrieves the json using the api and api token
fn get_data(token: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    Ok(response.text()?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
