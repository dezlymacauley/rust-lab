/*
    ABOUT: Traits

    Think of a trait as a block that defines one or more flexible rules.
    A trait can be attached to multiple structs.

    For example:

    trait Shape {
        fn area(&self) -> f32;
        fn perimeter(&self) -> f32;
    }

    This is a trait called `Shape`,
    and it has two rules.

    1. Any struct that you attach this trait to,
    must have a method called `area` that returns an `f32` data type.

    3. Any struct that you attach this trait to,
    must have a method called `perimeter` that returns an `f32` data type.

    The inner workings of ther `area` and perimeter functions do not matter,
    as long as they satify the rules of the trait.

    In simple terms, a struct that has the `trait` `Shape`,
    must have an `area` method and a `perimeter` method.
*/

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
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

// The constructor function goes into a regular `impl` block
// because each struct requires specific parameters to create an instance.
impl Square {
    fn new(side: f32, line_width: u8, color: &str) -> Self {
        Self {
            side: side,
            line_width: line_width,
            color: color.to_string(),
        }
    }
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

impl Rectangle {
    fn new(length: f32, width: f32, line_width: u8, color: &str) -> Self {
        Self {
            length: length,
            width: width,
            line_width: line_width,
            color: color.to_string(),
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rectangle = self.length * self.width;
        println!("area_of_rectangle: {area_of_rectangle}");
        area_of_rectangle
    }
}

fn main() {
    let square_a: Square = Square::new(6.25, 2, "red");
    let rectangle_a: Rectangle = Rectangle::new(4.25, 5.75, 3, "blue");

    square_a.area();
    rectangle_a.area();
}
