//A collection of functions for accessing the canvas api
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

// So it looks like I will need to make structures to directly manipulate the json data.
// I will just implement what I need for now, and add more later if I need to.
// Todo expand user struct for greeting
#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
}

// TODO This struct is for the json data returned from the courses endpoint
// TODO Fields are: name (String), overall grade (Float), letter grade (String), and course id (int)

// Since I am serializing this, I will need to implement as many fields the team will need to
// access.
// WIP
#[derive(Serialize, Deserialize)]
pub struct Assignment {
    #[serde(rename = "id")]
    pub assignment_id: i32,
    #[serde(rename = "name")]
    pub assignment_name: String,
    #[serde(rename = "points_possible")]
    pub possible_pts: Option<f32>,
    #[serde(rename = "due_at")]
    pub assignment_due: Option<String>,
    pub submission: Option<Submission>, //added post serialization
}

// Actual submission data for an assignment, this is a sub-struct of Assignment
#[derive(Serialize, Deserialize)]
pub struct Submission {
    #[serde(rename = "submission_id")]
    pub submission_id: i32,
    #[serde(rename = "submitted_at")]
    pub assignment_created: String,
    #[serde(rename = "score")]
    pub points_earned: Option<f32>,
}

//uhh I think this works
pub fn get_apikey() -> String {
    env::var("CANVAS_API_KEY").unwrap_or("".to_string())
}

//returns a user struct with the user's id and name
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

//returns true if the key is invalid
pub fn validate_key(key: &str) -> bool {
    let mut valid = true;
    match get_data(
        &key,
        "https://alamo.instructure.com/api/v1/users/self/profile",
    ) {
        Ok(body) => {
            if !body.contains("Invalid access key") {
                valid = false;
            }
            println!("{}", body);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    valid
}

// Returns a json string of the user's currently enrolled courses
// TODO Reformat into a struct for field manipulation, name is not connected to overall grade
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
    return "Invalid course data".to_string();
}

// TODO This function should return a cobbled together json of assignments and grades
// Need to write a loop to go through all pages of assignments and call the submissions endpoint

pub fn retrieve_assignments(key: &str, course_id: i32) -> Vec<Assignment> {
    let mut assignmentlist: Vec<Assignment> = Vec::new();
    let mut page_number = 1;
    //loop through pages of assignments
    loop {
        let url = format!(
            "https://alamo.instructure.com/api/v1/courses/{}/assignments?page={}",
            course_id, page_number
        );

        match get_data(&key, &url) {
            Ok(body) => {
                let mut page_of_assignments: Vec<Assignment> = serde_json::from_str(&body).unwrap();
                if page_of_assignments.is_empty() {
                    // No more assignments, so break out of the loop
                    break;
                }
                //TODO here we need to get the grade for each assignment, so make an api call to the submissions endpoint
                //loop through the vec of assignments, retrieve the id of each assignment, and make a call to the submissions endpoint
                //from there, append the grade to the assignment struct
                //then append the assignment struct to the assignmentlist vec
                assignmentlist.append(&mut page_of_assignments);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        page_number += 1;
    }
    return assignmentlist; //returns a vector of assignments all with a grade of "None" for now...
}

//baseline for authentication
fn get_data(key: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", key))
        .send()?;
    Ok(response.text()?)
}
