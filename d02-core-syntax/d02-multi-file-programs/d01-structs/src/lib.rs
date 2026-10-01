#[derive(Debug)]
pub struct Student {
    // The `id` field is private
    // This is useful for creating a field with a value that should not be
    // manually set when an instance of the struct is created.
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

    // Because the `id` field is private,
    // you need a method to be able to read the value of id
    pub fn id(&self) -> u8 {
        self.id
    }
}
