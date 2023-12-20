use crate::canvas;
use canvas::canvas::{canvas_get_assignments, canvas_get_courses, canvas_get_user};

pub fn test_canvas_functions() {
    let user = canvas_get_user();
    let courses = canvas_get_courses(user.id);
    let mut assignments = canvas_get_assignments(1567838);
    for assignment in &mut assignments {
        match &assignment.letter_grade {
            Some(grade) => println!("Grade: {}", grade),
            None => println!("No grade"),
        }
    }
    let name: &str = &user.name;
    println!("Hello from rust! Your name is {}", name);
    println!("Courses: {:?}", courses);
    println!("Assignments: {:?}", assignments);
}
