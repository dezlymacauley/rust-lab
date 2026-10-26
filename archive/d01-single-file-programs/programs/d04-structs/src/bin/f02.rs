#[derive(Debug)]
pub struct Student {
    id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(student_name: &str) -> Self {
        Self {
            id: 0,
            age: 20,
            name: String::from(student_name),
        }
    }
}

fn main() {
    let student_one: Student = Student::new("Dezly");
    println!("\nstudent_one: {student_one:#?}");

    // println!("\nstudent_one.id: {}", student_one.id());
    
    println!("\nstudent_one.id: {}", student_one.id);
    println!("\nstudent_one.name: {}", student_one.name);
    println!("\nstudent_one.age: {}", student_one.age);
}
