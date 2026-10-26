/*
    ABOUT: Traits

    Think of a trait as a block that defines one or more flexible rules.
    A trait can be attached to multiple structs.

    For example:

    trait Shape {
        fn area(&self) -> f32;
    }

    This is a trait called `Shape`,
    and it has one rule.

    Any struct that you attach this trait to,
    must have a method called `area` that returns an `f32` data.

    The inner workings of that `area` function do not matter,
    as long as it satifies the condition of the trait.
*/

trait Shape {
    fn area(&self) -> f32;
}

struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

// This is how you add a trait to struct
impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("area_of_square: {area_of_square}");
        area_of_square
    }
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rectangle = self.length * self.width;
        println!("area_of_rectangle: {area_of_rectangle}");
        area_of_rectangle
    }
}

fn main() {}
