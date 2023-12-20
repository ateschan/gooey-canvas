use crate::canvas::canvas::get_data;
use std::env;

pub fn set_apikey(key: &str) {
    env::set_var("CANVAS_API_KEY", key);
}

pub fn get_apikey() -> String {
    env::var("CANVAS_API_KEY").unwrap_or("".to_string())
}

pub fn validate_key(key: &str) -> bool {
    let mut valid = true;
    match get_data(
        &key,
        "https://alamo.instructure.com/api/v1/users/self/profile",
    ) {
        Ok(body) => {
            if body.contains("Invalid access token.") || body.contains("unauthenticated") {
                valid = false;
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    valid
}
