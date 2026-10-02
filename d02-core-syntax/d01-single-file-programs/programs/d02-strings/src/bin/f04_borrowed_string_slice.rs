fn main() {
    let mut s1: &str = "Dezly";
    let s2: &str = s1;

    println!("{:p}", s1.as_ptr());
    println!("{:p}", s2.as_ptr());
    // 0x55b7f97bcf40
    // 0x55b7f97bcf40

    println!("s1: {s1}");
    println!("s2: {s2}");
    // s1: Dezly
    // s2: Dezly
    //_________________________________________________________________________

    s1 = "Seth";

    println!("s1: {s1}");
    println!("s2: {s2}");
    // s1: Seth
    // s2: Dezly

    println!("{:p}", s1.as_ptr());
    println!("{:p}", s2.as_ptr());
    // 0x55b7f97be5a0
    // 0x55b7f97bcf40

    //_________________________________________________________________________
}
