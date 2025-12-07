use super::weapon::{Weapon, WeaponCategory, WeaponInfo};
use el_roi::read_int;
use serde::{Deserialize, Serialize};
use std::fmt;

const FIGHT_STYLE_GUIDE: &str =
    "Fighting styles define stance bonuses and unlock curated weapon pools tied to each archetype.";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FightingStyle {
    SwordsMan,
    DualBladeWelder,
    Tank,
    Caster,
    Ranger,
    Brawler,
}

impl fmt::Display for FightingStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FightingStyle::SwordsMan => "Sword Master",
            FightingStyle::DualBladeWelder => "Dual Blade",
            FightingStyle::Tank => "Tank",
            FightingStyle::Caster => "Caster",
            FightingStyle::Ranger => "Ranger",
            FightingStyle::Brawler => "Brawler",
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        let style = None;
        let weapons = WeaponInfo::new_weapons();

        Self { style, weapons }
    }

    #[allow(dead_code)]
    pub fn style(&self) -> Option<&FightingStyle> {
        self.style.as_ref()
    }

    #[allow(dead_code)]
    pub fn weapons(&self) -> &[Weapon] {
        &self.weapons
    }

    pub fn from_prompt() -> Self {
        println!("--- Fighting Style ---");
        println!("{}", FIGHT_STYLE_GUIDE);
        let options = style_options();
        for (idx, style) in &options {
            println!("{}. {}", idx, style);
        }
        let selection = read_int("Pick your primary style: ");
        let style = options
            .iter()
            .find(|(idx, _)| *idx == selection)
            .map(|(_, style)| *style)
            .unwrap_or(FightingStyle::SwordsMan);
        let weapons = weapons_for(style);
        Self {
            style: Some(style),
            weapons,
        }
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

fn style_options() -> Vec<(i32, FightingStyle)> {
    vec![
        (1, FightingStyle::SwordsMan),
        (2, FightingStyle::DualBladeWelder),
        (3, FightingStyle::Tank),
        (4, FightingStyle::Caster),
        (5, FightingStyle::Ranger),
        (6, FightingStyle::Brawler),
    ]
}

fn weapons_for(style: FightingStyle) -> Vec<Weapon> {
    let allowed_categories = match style {
        FightingStyle::SwordsMan => vec![WeaponCategory::Swordsman],
        FightingStyle::DualBladeWelder => vec![WeaponCategory::DualBlade],
        FightingStyle::Tank => vec![WeaponCategory::Tank],
        FightingStyle::Caster => vec![WeaponCategory::Caster],
        FightingStyle::Ranger => vec![WeaponCategory::Ranger],
        FightingStyle::Brawler => vec![WeaponCategory::Brawler],
    };

    WeaponInfo::new_weapons()
        .into_iter()
        .filter(|weapon| allowed_categories.contains(&weapon.category()))
        .collect()
}
