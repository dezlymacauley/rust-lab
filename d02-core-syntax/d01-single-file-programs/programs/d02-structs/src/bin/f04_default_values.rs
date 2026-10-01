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

// `Default` is a trait that allows you to add a method called `default()`
// to a struct. This function allows you to create a struct
// with default values.
impl Default for Student {
    // NOTE: The function must be called `default`

    fn default() -> Self {
        Self {
            id: 0,
            age: 18,
            name: String::from("Cassie")
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
            age: 18,
            name: "Cassie",
        }

    */
    
    //_________________________________________________________________________
    
    // EXAMPLE: 2 => Overriding some of the default values 

    // I want to create an instance of a struct with



    //_________________________________________________________________________
}
