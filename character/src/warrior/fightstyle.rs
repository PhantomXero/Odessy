use super::weapon::{Weapon, WeaponInfo};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum FightingStyle {
    SwordsMan,
    DualBladeWelder,
    Tank,
    Caster,
}

impl fmt::Display for FightingStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FightingStyle::SwordsMan => "Sword Master",
            FightingStyle::DualBladeWelder => "Dual Blade",
            FightingStyle::Tank => "Tank",
            FightingStyle::Caster => "Caster",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightingStyleInfo {
    style: Option<FightingStyle>,
    weapons: Vec<Weapon>,
}

impl FightingStyleInfo {
    pub fn new() -> Self {
        let style = None;
        let weapons = WeaponInfo::new_weapons();

        Self { style, weapons }
    }

    pub fn style(&self) -> Option<&FightingStyle> {
        self.style.as_ref()
    }

    pub fn weapons(&self) -> &[Weapon] {
        &self.weapons
    }
}

impl fmt::Display for FightingStyleInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = self
            .style
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unassigned".to_string());
        let weapon_list = if self.weapons.is_empty() {
            "None".to_string()
        } else {
            self.weapons
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };
        write!(f, "Style: {style} | Weapons: {weapon_list}")
    }
}
