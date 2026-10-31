/*
    ABOUT: Traits and Polymorphism

    ___________________________________________________________________________

    In Rust, a `trait` is a mechanism that allows you to use polymorphism
    in your programs.
    ___________________________________________________________________________

    What is polymorphism?

    The word polymorphism comes from two greek words, `poly` and `morph`:
    - Poly means `many`
    - Morph means `forms`
    - So `polymorphism` means `many forms`

    ___________________________________________________________________________


    ___________________________________________________________________________
*/

#![allow(dead_code, unused_variables)]

//_____________________________________________________________________________

// SECTION: Mage Declaration

struct Mage {
    name: String,
    health: u32,
    spell_damage: u32,
    magic_pool: u32,
}

//_____________________________________________________________________________

// SECTION: Samurai Declaration

struct Samurai {
    name: String,
    health: u32,
    sword_damage: u32,
    blade_sharpness: u32,
}

//_____________________________________________________________________________

fn main() {
    let samurai_one: Samurai = Samurai {
        name: String::from("Kevin"),
        health: 100,
        sword_damage: 4522,
        blade_sharpness: 100,
    };

    let mage_one: Mage = Mage {
        name: String::from("Cassie"),
        health: 100,
        spell_damage: 6521,
        magic_pool: 100,
    };
}
