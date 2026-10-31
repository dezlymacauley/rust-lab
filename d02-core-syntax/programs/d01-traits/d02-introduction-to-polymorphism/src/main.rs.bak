/*
    ABOUT: Traits

    Think of a trait as a block that defines one or more flexible rules.
    A trait can be attached to multiple structs.

    ___________________________________________________________________________

    For example:

    trait Shape {
        fn print_shape_info(&self);
        fn area(&self) -> f32;
        fn perimeter(&self) -> f32;
    }

    This is a trait called `Shape`,
    and it has three rules.

    1. Any struct that you attach this trait to,
    must have a method called `print_shape_info` that does not return 
    any data type.

    2. Any struct that you attach this trait to,
    must have a method called `area` that returns an `f32` data type.

    3. Any struct that you attach this trait to,
    must have a method called `perimeter` that returns an `f32` data type.

    The inner workings of ther `area` and perimeter functions do not matter,
    as long as they satify the rules of the trait.

    ___________________________________________________________________________

    The benefits of a trait is that it allows you to create uniform API
    across different structs that should have similar functionality.

    It ensures that each struct has methods that are spelled the same way,
    and return the same data type. 

    ___________________________________________________________________________
*/

trait Shape {
    fn print_shape_info(&self);
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
    fn print_shape_info(&self) {
        println!();
        println!("Square:");
        println!("  side:       {}", self.side);
        println!("  line_width: {}", self.line_width);
        println!("  color:      {}", self.color);
    }

    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        area_of_square
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_square = self.side * 4.0;
        perimeter_of_square
    }
}

// The constructor function goes into a regular `impl` block
// because each struct requires specific parameters to create an instance
// of a specific struct
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

impl Shape for Rectangle {
    fn print_shape_info(&self) {
        println!();
        println!("Rectangle:");
        println!("  length:     {}", self.length);
        println!("  width:      {}", self.width);
        println!("  line_width: {}", self.line_width);
        println!("  color:      {}", self.color);
    }

    fn area(&self) -> f32 {
        let area_of_rectangle = self.length * self.width;
        area_of_rectangle
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rectangle = (self.length * 2.0) + (self.width * 2.0);
        perimeter_of_rectangle
    }
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


fn main() {
    let square_a: Square = Square::new(6.25, 2, "red");
    let rectangle_a: Rectangle = Rectangle::new(4.25, 5.75, 3, "blue");

    square_a.print_shape_info();
    rectangle_a.print_shape_info();

    println!("\narea of square_a: {}", square_a.area());
    println!("\narea of rectangle_a: {}", rectangle_a.area());

    println!("\nperimeter of square_a: {}", square_a.perimeter());
    println!("\nperimeter of rectangle_a: {}", rectangle_a.perimeter());

}
