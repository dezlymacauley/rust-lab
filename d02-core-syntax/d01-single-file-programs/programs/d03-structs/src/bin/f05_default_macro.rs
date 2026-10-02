#[derive(Debug, Default)]
pub struct Student {
    #[allow(dead_code)]
    id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(student_name: &str) -> Result<Self, String> {
        if student_name.chars().all(|x: char| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name: String::from(student_name),
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

fn main() {
    //_________________________________________________________________________

    // EXAMPLE: 1 => Creating an instance of a struct with default values

    let student_one: Student = Student::default();
    println!("\nstudent_one: {student_one:#?}");
    /*
        student_one: Student {
            id: 0,
            age: 0,
            name: "",
        }
    */

    //_________________________________________________________________________

    // EXAMPLE: 2 => Overriding some of the default values

    // I want to create an instance of a struct by manually setting the
    // `name` field, while letting the `default()` function set the values
    // for all of the remaining fields in the struct.

    let student_two = Student {
        name: String::from("Lisa"),
        ..Default::default()
    };
    println!("\nstudent_two: {student_two:#?}");
    /*
        student_two: Student {
            id: 0,
            age: 0,
            name: "Lisa",
        }
    */

    //_________________________________________________________________________
}
