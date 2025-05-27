use super::weapon::{self, Weapon, WeaponInfo};

#[derive(Debug)]
enum FightingStyle {
    SwordsMan,
    DualBladeWelder,
    Tank,
    Caster,
}

#[derive(Debug)]
pub struct FightingStyleInfo {
    style: Option<FightingStyle>,
    weapons: Vec<Weapon>,
}

impl FightingStyleInfo {
    pub fn new() -> Self {
        let style = None;
        let weapons = WeaponInfo::new_weapons();

        Self{style, weapons}
    }
}
