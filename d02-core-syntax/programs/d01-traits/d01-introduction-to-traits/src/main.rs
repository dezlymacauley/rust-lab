// SECTION: Square Declaration

struct Square {
    side: f32,
    line_width: u8,
    color: String
}

impl Square {
    fn calculate_area(&self) {
        println!("The area is: {}", self.side * self.side);
    }
}

//_____________________________________________________________________________

// SECTION: Rectangle Declaration

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String
}

impl Rectangle {
   fn area(&self) -> f32 {
       self.length * self.width
   }
}

//_____________________________________________________________________________

fn main() {

}
