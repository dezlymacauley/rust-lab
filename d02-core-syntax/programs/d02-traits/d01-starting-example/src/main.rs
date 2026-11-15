/*
    ABOUT: Starting Example

    ___________________________________________________________________________

    Before I explain what `polymorphism` is,
    or what a `trait` is, I'm going to start off with a simple example
    that does NOT use either of these concepts.
    ___________________________________________________________________________

    There are two structs: `Mage` and `Samurai`

    These structs are used to define the two playable character
    types in a game.

    ___________________________________________________________________________

    Each of struct has a method to calculate the attack damage
    of that specific player type.

    The Mage struct has:
    fn magic_attack_damage(&self)

    The Samurai struct has:
    fn sword_attack_damage(&self) -> u32

    ___________________________________________________________________________

    1. Both of these methods have different names
    2. These methods also have different return types.

    ___________________________________________________________________________

*/

#![allow(dead_code, unused_variables)]

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
