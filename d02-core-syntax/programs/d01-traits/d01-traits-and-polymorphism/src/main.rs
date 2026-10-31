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

// Adding this to silence the warning regarding unused code in this file
#![allow(dead_code)]

//_____________________________________________________________________________

// SECTION: Archer Declaration

struct Archer {
    name: String,
    health: u32,
    arrow_damage: u32,
    arrows: u32,
}

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

fn main() {}
