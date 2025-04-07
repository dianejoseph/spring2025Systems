// In class Assignment

// Create a struct Student (major)
struct Student {
    major:String,
}
// Higher order functions update majors

fn update_majors(mut collection: Vec<Student>,behavior:fn(&mut Student,String)){
    for student in &mut collection {
        behavior(student, "Computer Engineering".to_string());
    }

    for (i, student) in collection.iter().enumerate(){
        println!("Student {}: {}", i + 1, student.major);
    }
}
// First Order functions, assign_major(student,major_declared)

fn assign_major(s: &mut Student, major: String){
    s.major = major;
}

fn main(){
    let student1 = Student { major: String::new() };
    let student2 = Student { major: String::new() };
    let student3 = Student { major: String::new() };


// create a vector of students1,2,3 and update all students major
    let students = vec![student1, student2, student3];

    update_majors(students, assign_major);

}