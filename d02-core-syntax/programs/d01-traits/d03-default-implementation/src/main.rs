/*
    ABOUT: Default implementation

*/

#![allow(dead_code, unused_variables)]

//_____________________________________________________________________________

// SECTION: Trait Declaration

trait PlayableCharacter {
    fn attack_damage(&self) -> u32;

    // NOTE: This is a default implementation

    // You can tell that it is a default implementation because this it not
    // just a function signature, but a fully implemented method. 

    // Another difference is that a data type that is part of 
    // the `PlayableCharacter` group is considered valid even if it doesn't
    // have this method (unlike `attack_damage` which is required.)

    // This is useful because every PlayableCharacter will have a default
    // taunt.

    fn taunt_enemy(&self) {
        println!("Is that all you've got? Pathetic!");
    }

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
}

impl PlayableCharacter for Mage {
    fn attack_damage(&self) -> u32 {
        let magic_attack_damage: u32 = self.spell_strength + self.energy_level;
        magic_attack_damage
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
}

impl PlayableCharacter for Samurai {
    fn attack_damage(&self) -> u32 {
        let sword_attack_damage: u32 =
            self.sword_strength + self.blade_condition;
        sword_attack_damage
    }
}

//_____________________________________________________________________________

fn main() {
    let samurai_one: Samurai = Samurai::new("Kevin", 100, 4522, 100);
    let mage_one: Mage = Mage::new("Cassie", 100, 6521, 100);

    samurai_one.taunt_enemy();
    mage_one.taunt_enemy();
}
