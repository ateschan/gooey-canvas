use crate::canvas;

pub fn test_canvas_functions() {
    let user = canvas::get_user();
    let courses = canvas::get_courses(user.id);
    let mut assignments = canvas::get_assignments(1567838);
    for assignment in &mut assignments {
        match assignment.grade {
            Some(grade) => println!("Grade: {}", grade),
            None => println!("No grade"),
        }
    }
    let name: &str = &user.name;
    println!("Hello from rust! Your name is {}", name);
    println!("Courses: {:?}", courses);
    println!("Assignments: {:?}", assignments);
}
