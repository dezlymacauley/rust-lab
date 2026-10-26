use d01_structs::Student;

fn main() {
    let student_one: Student = Student::new("Dezly");
    println!("\nstudent_one: {student_one:#?}");

    println!("\nstudent_one.id: {}", student_one.id());
    
    println!("\nstudent_one.name: {}", student_one.name);
    println!("\nstudent_one.age: {}", student_one.age);
}
