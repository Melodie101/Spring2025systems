// In class Assignment
// Name: Melodie Aranda

// Create a struct Student (major)
struct Student {
    major:String,
}

// Higher order functions update majors
fn update_majors (collection: &mut Vec<Student>,behavior:fn(&mut Student,String)) {
    for student in collection.iter_mut() {
        behavior(student, "Computer Engineering".to_string());
    }
}

// First Order functions, assign_major(student,major_declared)
fn assign_major(student: &mut Student, major: String){
    student.major = major;

}


fn main(){
    
// create a vector of students1,2,3 and update all students major
    let mut students123 = vec![
        Student { major: "Electrical Engineering".to_string() },
        Student { major: "Architecture".to_string() },
        Student { major: "Computer Science".to_string() },
    ];

    update_majors(&mut students123, assign_major);

    // Printing the updated majors
    for student in students123 {
        println!("Student major: {}", student.major);
    }

}