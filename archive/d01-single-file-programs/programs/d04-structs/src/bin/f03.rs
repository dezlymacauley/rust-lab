#[derive(Debug)]
pub struct Student {
    #[allow(dead_code)]
    id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x: char| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name: std_name,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

fn main() {
    let student_one = Student::new("dez123".to_string());
    // let student_one: Student = Student::new("Dezly");
    println!("\nstudent_one: {student_one:#?}");

    // println!("\nstudent_one.id: {}", student_one.id());
    
    // println!("\nstudent_one.id: {}", student_one.id);
    // println!("\nstudent_one.name: {}", student_one.name);
    // println!("\nstudent_one.age: {}", student_one.age);
}
