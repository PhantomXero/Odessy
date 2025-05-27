#[derive(Debug)]
pub enum Weapon {
    LongSword,
    ShortSword,
    DualLongSword,
    DualShortSword,
    Axe,
    Hummer,
    Hands,
}

#[derive(Debug)]
pub struct WeaponInfo {
    class: Weapon,
    damage: f64,
    durability: f64,
}

impl WeaponInfo {
    pub fn new() -> Self {
        let class = Weapon::Hands;
        let damage = 0.0;
        let durability = 0.0;

        Self{class, damage, durability}
    }

    pub fn new_weapons() -> Vec<Weapon> {
        let mut weapons = Vec::new();
        weapons.push(Weapon::Hands);
        weapons
    }
}