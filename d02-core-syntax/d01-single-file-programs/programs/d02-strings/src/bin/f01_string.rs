fn main() {

    // EXAMPLE: 1 => Rust ensures that the String has only one owner.

    let s1: String = String::from("Dezly");
    let s2 = s1;

    // This will not work
    // println!("s1: {s1}");

    //_________________________________________________________________________

    // s1 = String::from("Seth");

    // println!("s1: {s1}");
    // println!("s2: {s2}");
    // s1: Seth
    // s2: Dezly

    // println!("{:p}", s1.as_ptr());
    // println!("{:p}", s2.as_ptr());
    // Different addresses

    //_________________________________________________________________________
}
