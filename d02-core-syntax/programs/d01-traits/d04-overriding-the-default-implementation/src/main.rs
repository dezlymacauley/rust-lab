/*
    ABOUT: Overriding the Default implementation

*/

#![allow(dead_code, unused_variables)]

//_____________________________________________________________________________

// SECTION: Trait Declaration

trait PlayableCharacter {
    fn attack_damage(&self) -> u32;

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


    // NOTE: I'm overriding the default `taunt_enemy` method
    
    // This allows me to create a custom taunt for the Samurai struct

    fn taunt_enemy(&self) {
        println!("I don't even need my sword to beat you");
    }
}

//_____________________________________________________________________________

fn main() {
    let samurai_one: Samurai = Samurai::new("Kevin", 100, 4522, 100);
    let mage_one: Mage = Mage::new("Cassie", 100, 6521, 100);

    samurai_one.taunt_enemy();
    mage_one.taunt_enemy();
    // I don't even need my sword to beat you
    // Is that all you've got? Pathetic!
}
