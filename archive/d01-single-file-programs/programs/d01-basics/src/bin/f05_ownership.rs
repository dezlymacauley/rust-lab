fn main() {
    let s1: &str = "Dezly";
    let s2: &str = s1;

    println!("{:p}", s1.as_ptr());
    println!("{:p}", s2.as_ptr());
    // These should be at te same memory address

    println!("s1: {s1}");
    println!("s2: {s2}");
    // s1: Dezly
    // s2: Dezly
}
