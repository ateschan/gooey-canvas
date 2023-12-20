//A collection of functions for accessing the canvas api, api key  validation and grade calculation
//are also perfromed here

use crate::canvas::canvas::{Assignment, Course, Grades, Submission, User};

pub fn return_mockedcourses(_num: i32) -> Vec<Course> {
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

pub fn return_mockedassignments(_num: i32) -> Vec<Assignment> {
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
