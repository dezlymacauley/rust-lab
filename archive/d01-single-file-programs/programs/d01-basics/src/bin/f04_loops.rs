fn main() {

    // EXAMPLE: 1 => Infinite loop with `break` keyword
  
    let mut apples_picked: u8 = 0;

    loop {

        if apples_picked == 2 {
            break;
        }
        
        apples_picked += 1;
        println!("apples_picked: {apples_picked}");
    }

    println!("Two apples have been picked");

    //_________________________________________________________________________
   
    // EXAMPLE: 2 => While loop
 
    let mut grapes_available: u8 = 4;

    while grapes_available != 0 {
        grapes_available -= 1;
        println!("grapes_available: {grapes_available}");
    }

    //_________________________________________________________________________
    
    // EXAMPLE: 3 => For in loop (index and element)

    // index:                      0   1   2   3   4
    let numbers_list: [u16; 5] = [17, 14, 10, 42, 82];

    for (index, element) in numbers_list.iter().enumerate() {
        println!("index {index} is {element}");
    }

    // index 0 is 17
    // index 1 is 14
    // index 2 is 10
    // index 3 is 42
    // index 4 is 82

    //_________________________________________________________________________
   
    // EXAMPLE: 4 => For in loop (Only the element)

    // index:                      0   1   2   3   4
    let numbers_list: [u16; 5] = [17, 14, 10, 42, 82];

    for element in numbers_list {
        println!("{element}");
    }

    // 17
    // 14
    // 10
    // 42
    // 82

    //_________________________________________________________________________

    // EXAMPLE: 5 => For in loop for repetition (inclusive range)

    for _ in 1..=3 {
        println!("Rust is awesome!");
    }

    // Rust is awesome!
    // Rust is awesome!
    // Rust is awesome!
    //_________________________________________________________________________

}
