//A collection of functions for accessing the canvas api, api key  validation and grade calculation
//are also perfromed here
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Serialize, Debug, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "avatar_url")]
    pub pic: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Course {
    #[serde(rename = "course_id")]
    pub id: i32,
    #[serde(rename = "created_at")]
    pub date: String,
    #[serde(rename = "grades")]
    pub grades: Grades,
    pub course_name: Option<String>, // added post deserialization
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Grades {
    #[serde(rename = "current_grade")]
    pub letter_grade: Option<String>,
    #[serde(rename = "current_score")]
    pub number_grade: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct CourseInfo {
    pub name: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Assignment {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "points_possible")]
    pub pts: Option<f32>,
    #[serde(rename = "due_at")]
    pub due: Option<String>,

    pub num_grade: Option<f32>,

    pub letter_grade: Option<String>, // added post deserialization
    pub submission: Option<Submission>,
}

#[derive(Clone, PartialEq, Serialize, Debug, Deserialize)]
pub struct Submission {
    #[serde(rename = "id")]
    pub submission_id: i32,
    #[serde(rename = "score")]
    pub pts: Option<f32>,
}

//Settting env variables
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

pub fn get_user() -> User {
    let mut user = User {
        id: 0,
        name: "".to_string(),
        pic: "".to_string(),
    };
    println!("API KEY: {}", get_apikey());
    match get_data(
        &get_apikey(),
        "https://alamo.instructure.com/api/v1/users/self/profile",
    ) {
        Ok(body) => {
            user = serde_json::from_str(&body).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    user
}

// Needs to cycle through pages to return the full list of courses, shouldn't be nessesary for most users
pub fn get_courses(user_id: i32) -> Vec<Course> {
    match get_data(
        &get_apikey(),
        &format!(
            "https://alamo.instructure.com/api/v1/users/{}/enrollments",
            user_id
        ),
    ) {
        Ok(body) => {
            let mut courses: Vec<Course> = serde_json::from_str(&body).unwrap();
            for course in &mut courses {
                course.course_name = Some(find_name(course.id));
                if course.grades.letter_grade == None {
                    course.grades.letter_grade =
                        Some(calculate_grade(course.grades.number_grade.unwrap()));
                }
            }

            return courses;
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return Vec::new();
        }
    }
}

fn find_name(course_id: i32) -> String {
    let mut name = "".to_string();

    match get_data(
        &get_apikey(),
        &format!("https://alamo.instructure.com/api/v1/courses/{}", course_id),
    ) {
        Ok(body) => {
            let course: CourseInfo = serde_json::from_str(&body).unwrap();
            name = course.name;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    return name;
}

// TODO This function should return a vec of assignments with added grade data
// Need to write a loop to go through all pages of assignments and call the submissions endpoint
// still need grade data
pub fn get_assignments(course_id: i32) -> Vec<Assignment> {
    let mut assignmentlist: Vec<Assignment> = Vec::new();
    let mut page_number = 1;
    loop {
        let url = format!(
            "https://alamo.instructure.com/api/v1/courses/{}/assignments?page={}",
            course_id, page_number
        );

        match get_data(&get_apikey(), &url) {
            Ok(body) => {
                let mut assignments: Vec<Assignment> = serde_json::from_str(&body).unwrap();
                if assignments.is_empty() {
                    // No more assignments, so break out of the loop
                    break;
                }
                for assignment in assignments.iter_mut() {
                    assignment.submission = Some(get_submission(course_id, assignment.id));
                    let sub = assignment.submission.clone();
                    if sub.clone().unwrap().pts == None {
                        assignment.letter_grade = Some("N/A".to_string());
                    } else if assignment.pts == None {
                        assignment.letter_grade = Some("N/A".to_string());
                    } else if &assignment.pts.unwrap() == &0.0 {
                        assignment.letter_grade = Some("N/A".to_string());
                        assignment.num_grade = Some(0.0);
                    } else {
                        assignment.letter_grade = Some(calculate_grade(calculate_score(
                            &mut sub.clone().unwrap().pts.unwrap(),
                            assignment.pts.unwrap(),
                        )));
                        assignment.num_grade = Some(calculate_score(
                            &mut sub.clone().unwrap().pts.unwrap(),
                            assignment.pts.unwrap(),
                        ));
                    }
                }
                assignmentlist.append(&mut assignments);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        page_number += 1;
    }
    return assignmentlist;
}

fn get_submission(course_id: i32, assignment_id: i32) -> Submission {
    let mut submission = Submission {
        submission_id: 0,
        pts: None,
    };
    match get_data(
        &get_apikey(),
        &format!(
            "https://alamo.instructure.com/api/v1/courses/{}/assignments/{}/submissions/self",
            course_id, assignment_id
        ),
    ) {
        Ok(body) => {
            submission = serde_json::from_str(&body).unwrap();
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    submission
}

fn calculate_score(pts: &mut f32, max_pts: f32) -> f32 {
    let score: f32;
    if pts == &mut 0.0 {
        score = 0.0;
    } else {
        score = *pts / max_pts;
    }
    return score * 100.0;
}

// add calculate grade function
fn calculate_grade(pts: f32) -> String {
    let grade: String;
    if pts >= 90.0 {
        grade = "A".to_string();
    } else if pts >= 80.0 {
        grade = "B".to_string();
    } else if pts >= 70.0 {
        grade = "C".to_string();
    } else if pts >= 60.0 {
        grade = "D".to_string();
    } else {
        grade = "F".to_string();
    }
    return grade;
}

fn get_data(key: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", key))
        .send()?;
    Ok(response.text()?)
}
