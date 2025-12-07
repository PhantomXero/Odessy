use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weapon {
    LongSword,
    ShortSword,
    DualLongSword,
    DualShortSword,
    Axe,
    Hummer,
    Hands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponInfo {
    class: Weapon,
    damage: f64,
    durability: f64,
}

impl WeaponInfo {
    pub fn new() -> Self {
        let class = Weapon::Hands;
        let damage = 0.1;
        let durability = 0.1;

        Self {
            class,
            damage,
            durability,
        }
    }

    pub fn new_weapons() -> Vec<Weapon> {
        let mut weapons = Vec::new();
        weapons.push(Weapon::Hands);
        weapons
    }

    pub fn class(&self) -> &Weapon {
        &self.class
    }

    pub fn damage(&self) -> f64 {
        self.damage
    }

    pub fn durability(&self) -> f64 {
        self.durability
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Weapon::LongSword => "Long Sword",
            Weapon::ShortSword => "Short Sword",
            Weapon::DualLongSword => "Dual Long Sword",
            Weapon::DualShortSword => "Dual Short Sword",
            Weapon::Axe => "Axe",
            Weapon::Hummer => "Hammer",
            Weapon::Hands => "Hands",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for WeaponInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Damage {:.1}, Durability {:.1})",
            self.class, self.damage, self.durability
        )
    }
}
