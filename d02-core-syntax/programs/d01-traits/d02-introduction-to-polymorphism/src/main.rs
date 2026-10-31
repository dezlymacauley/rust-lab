/*
    ABOUT: Introduction to polymorphism 

    ___________________________________________________________________________

    The word polymorphism comes from two greek words: `poly` and `morph`:

    `poly` means many 

    `morph` means forms

    So therefore polymorphism means `many forms`
    ___________________________________________________________________________

    You can use polymorphism in your Rust programs by using a mechanism
    called a trait
    ___________________________________________________________________________

*/

#![allow(dead_code, unused_variables)]

//_____________________________________________________________________________

// SECTION: Trait Declaration

// Think of the `Mage` and `Samurai` structs as templates for creating
// a specific type of playable character in a game.

// Think of a `trait` as a membership card that can be given 
// to multiple data types in Rust.

// This membership card contains a set of rules that each member of the group
// must follow.

// E.g. I want to create a trait called `PlayableCharacter` and any
// data type that has this membership card must have a method to calculate
// their attack damage

// member
// This is a `trait` called `PlayableCharacter`
//
//
// Anything data type that has this membership card,
// must have a method called `attack_damage` that returns a u32
// The logic inside the `attack_damage` function does not matter.
trait PlayableCharacter {
    fn attack_damage(&self) -> u32;
}

//_____________________________________________________________________________

// SECTION: Mage Declaration

struct Mage {
    name: String,
    health: u32,
    spell_strength: u32,
    energy_level: u32,
}

impl Mage {
    fn new(
        name: &str,
        health: u32,
        spell_strength: u32,
        energy_level: u32,
    ) -> Self {
        Self {
            name: String::from(name),
            health,
            spell_strength,
            energy_level,
        }
    }

    fn magic_attack_damage(&self) {
        let magic_attack_damage: u32 = self.spell_strength + self.energy_level;
        println!("Magic Attack Damage: {}", magic_attack_damage);
    }
}

//_____________________________________________________________________________

// SECTION: Samurai Declaration

struct Samurai {
    name: String,
    health: u32,
    sword_strength: u32,
    blade_condition: u32,
}

impl Samurai {
    fn new(
        name: &str,
        health: u32,
        sword_strength: u32,
        blade_condition: u32,
    ) -> Self {
        Self {
            name: String::from(name),
            health,
            sword_strength,
            blade_condition,
        }
    }

    fn sword_attack_damage(&self) -> u32 {
        let sword_attack_damage: u32 =
            self.sword_strength + self.blade_condition;

        sword_attack_damage
    }
}

//_____________________________________________________________________________

fn main() {
    let samurai_one: Samurai = Samurai::new("Kevin", 100, 4522, 100);
    let mage_one: Mage = Mage::new("Cassie", 100, 6521, 100);

    println!(
        "Sword attack damage: {}",
        samurai_one.sword_attack_damage()
    );

    mage_one.magic_attack_damage();

}
