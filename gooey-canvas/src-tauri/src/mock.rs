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
pub fn return_mockedcourses(num: i32) -> Vec<Course> {
    let mockcourses: Vec<Course> = vec![
        Course {
            id: 1,
            date: "2021-01-01T00:00:00Z".to_string(),
            grades: Grades {
                letter_grade: Some("A".to_string()),
                number_grade: Some(100.0),
            },
            course_name: Some("Course 1".to_string()),
        },
        Course {
            id: 2,
            date: "2021-01-01T00:00:00Z".to_string(),
            grades: Grades {
                letter_grade: Some("B".to_string()),
                number_grade: Some(90.0),
            },
            course_name: Some("Course 2".to_string()),
        },
        Course {
            id: 3,
            date: "2021-01-01T00:00:00Z".to_string(),
            grades: Grades {
                letter_grade: Some("C".to_string()),
                number_grade: Some(80.0),
            },
            course_name: Some("Course 3".to_string()),
        },
        Course {
            id: 4,
            date: "2021-01-01T00:00:00Z".to_string(),
            grades: Grades {
                letter_grade: Some("D".to_string()),
                number_grade: Some(70.0),
            },
            course_name: Some("Course 4".to_string()),
        },
        Course {
            id: 5,
            date: "2021-01-01T00:00:00Z".to_string(),
            grades: Grades {
                letter_grade: Some("F".to_string()),
                number_grade: Some(60.0),
            },
            course_name: Some("Course 5".to_string()),
        },
    ];

    return mockcourses;
}

pub fn return_mockedassignments(num: i32) -> Vec<Assignment> {
    let mockassignments: Vec<Assignment> = vec![
        Assignment {
            id: 1,
            name: "Assignment 1".to_string(),
            pts: Some(100.0),
            due: Some("2021-01-01T00:00:00Z".to_string()),
            num_grade: Some(100.0),
            letter_grade: Some("A".to_string()),
            submission: Some(Submission {
                submission_id: 1,
                pts: Some(100.0),
            }),
        },
        Assignment {
            id: 2,
            name: "Assignment 2".to_string(),
            pts: Some(100.0),
            due: Some("2021-01-01T00:00:00Z".to_string()),
            num_grade: Some(100.0),
            letter_grade: Some("A".to_string()),
            submission: Some(Submission {
                submission_id: 2,
                pts: Some(100.0),
            }),
        },
        Assignment {
            id: 3,
            name: "Assignment 3".to_string(),
            pts: Some(100.0),
            due: Some("2021-01-01T00:00:00Z".to_string()),
            num_grade: Some(100.0),
            letter_grade: Some("A".to_string()),
            submission: Some(Submission {
                submission_id: 3,
                pts: Some(100.0),
            }),
        },
        Assignment {
            id: 4,
            name: "Assignment 4".to_string(),
            pts: Some(100.0),
            due: Some("2021-01-01T00:00:00Z".to_string()),
            num_grade: Some(100.0),
            letter_grade: Some("A".to_string()),
            submission: Some(Submission {
                submission_id: 4,
                pts: Some(100.0),
            }),
        },
        Assignment {
            id: 5,
            name: "Assignment 5".to_string(),
            pts: Some(100.0),
            due: Some("2021-01-01T00:00:00Z".to_string()),
            num_grade: Some(100.0),
            letter_grade: Some("A".to_string()),
            submission: Some(Submission {
                submission_id: 5,
                pts: Some(100.0),
            }),
        },
    ];
    return mockassignments;
}
pub fn get_user() -> User {
    let mockuser = User {
        id: 1,
        name: "Mock User".to_string(),
        pic: "https://mockuser.com".to_string(),
    };
    return mockuser;
}
