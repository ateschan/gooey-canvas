use reqwest;
use serde::Deserialize;
use serde_json;
use std::env;

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
}

pub fn get_apikey() -> String {
    env::var("CANVAS_API_KEY").unwrap_or("".to_string())
}

pub fn get_user(key: &str) -> User {
    let mut user = User {
        id: 1,
        name: "".to_string(),
    };
    match get_data(
        &key,
        "https://alamo.instructure.com/api/v1/users/self/profile",
    ) {
        Ok(body) => {
            user = serde_json::from_str(&body).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    user
}

pub fn validate_key(key: &str) -> bool {
    let mut valid = true;
    match get_data(
        &key,
        "https://alamo.instructure.com/api/v1/users/self/profile",
    ) {
        Ok(body) => {
            if !body.contains("Invalid access token") {
                valid = false;
            }
            println!("{}", body);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    valid
}

pub fn get_courses(key: &str) -> String {
    let user = get_user(&key);
    let url = format!(
        "https://alamo.instructure.com/api/v1/users/{}/enrollments",
        user.id
    );
    match get_data(&key, &url) {
        Ok(body) => {
            println!("{}", body);
            return body;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    return "".to_string();
}

fn get_data(token: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    Ok(response.text()?)
}
