use crate::physical::physical::PhysicalTier;
use crate::prompt::{select_from_menu, MenuItem};
use super::weapon::{weapon_catalog, weapon_variants, WeaponArchetype, WeaponDefinition, WeaponFamily};
use serde::{Deserialize, Serialize};
use std::fmt;

const FIGHT_STYLE_GUIDE: &str =
    "Fighting styles define stance bonuses and unlock curated weapon pools tied to each archetype.";

#[derive(Debug, Clone, Copy)]
struct FamilyUnlock {
    family: WeaponFamily,
    min_tier: PhysicalTier,
}

impl FamilyUnlock {
    const fn new(family: WeaponFamily, min_tier: PhysicalTier) -> Self {
        Self { family, min_tier }
    }

    fn is_unlocked(&self, tier: PhysicalTier) -> bool {
        tier >= self.min_tier
    }
}

const SWORDSMAN_UNLOCKS: &[FamilyUnlock] = &[
    FamilyUnlock::new(WeaponFamily::ShortBlade, PhysicalTier::Drifter),
    FamilyUnlock::new(WeaponFamily::LongBlade, PhysicalTier::Vanguard),
];

const DUAL_UNLOCKS: &[FamilyUnlock] = &[
    FamilyUnlock::new(WeaponFamily::DualShortBlade, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::DualDagger, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::DualLongBlade, PhysicalTier::Titan),
];

const TANK_UNLOCKS: &[FamilyUnlock] = &[
    FamilyUnlock::new(WeaponFamily::Shield, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::Axe, PhysicalTier::Vanguard),
    FamilyUnlock::new(WeaponFamily::Hammer, PhysicalTier::Vanguard),
    FamilyUnlock::new(WeaponFamily::Maul, PhysicalTier::Titan),
];

const CASTER_UNLOCKS: &[FamilyUnlock] = &[
    FamilyUnlock::new(WeaponFamily::Wand, PhysicalTier::Drifter),
    FamilyUnlock::new(WeaponFamily::Codex, PhysicalTier::Drifter),
    FamilyUnlock::new(WeaponFamily::Staff, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::Focus, PhysicalTier::Vanguard),
];

const RANGER_UNLOCKS: &[FamilyUnlock] = &[
    FamilyUnlock::new(WeaponFamily::ShortBow, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::LongBow, PhysicalTier::Vanguard),
    FamilyUnlock::new(WeaponFamily::Crossbow, PhysicalTier::Titan),
];

const BRAWLER_UNLOCKS: &[FamilyUnlock] = &[
    FamilyUnlock::new(WeaponFamily::Unarmed, PhysicalTier::Drifter),
    FamilyUnlock::new(WeaponFamily::Gauntlet, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::Dagger, PhysicalTier::Strider),
    FamilyUnlock::new(WeaponFamily::Whip, PhysicalTier::Vanguard),
];

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
    weapons: Vec<String>,
}

impl FightingStyleInfo {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let style = None;
        let weapons = weapon_catalog()
            .iter()
            .map(|definition| definition.weapon_name.to_string())
            .collect();

        Self { style, weapons }
    }

    #[allow(dead_code)]
    pub fn style(&self) -> Option<&FightingStyle> {
        self.style.as_ref()
    }

    #[allow(dead_code)]
    pub fn weapons(&self) -> &[String] {
        &self.weapons
    }

    pub fn from_prompt(current_tier: PhysicalTier) -> Self {
        println!("--- Fighting Style ---");
        println!("{}", FIGHT_STYLE_GUIDE);
        let entries = style_entries();
        let menu: Vec<MenuItem> = entries.iter().map(|(_, item)| item.clone()).collect();
        let selection = select_from_menu("Pick your primary style:", Some(FIGHT_STYLE_GUIDE), &menu);
        let style = entries[selection.index].0;
        println!("Fighting Style: {}", style);
        let weapons = weapons_for(style, current_tier);
        Self {
            style: Some(style),
            weapons,
        }
    }

    pub fn primary_style(&self) -> Option<FightingStyle> {
        self.style
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

fn style_entries() -> Vec<(FightingStyle, MenuItem)> {
    vec![
        (
            FightingStyle::SwordsMan,
            MenuItem::with_info(
                FightingStyle::SwordsMan.to_string(),
                "Sword Master: single-blade precision. Pros: balanced reach/control. Cons: limited burst vs armor.",
            ),
        ),
        (
            FightingStyle::DualBladeWelder,
            MenuItem::with_info(
                FightingStyle::DualBladeWelder.to_string(),
                "Dual Blade: relentless flurries with paired weapons. Pros: tempo control. Cons: stamina heavy.",
            ),
        ),
        (
            FightingStyle::Tank,
            MenuItem::with_info(
                FightingStyle::Tank.to_string(),
                "Tank: shields and impact weapons. Pros: mitigation and crowd control. Cons: low mobility.",
            ),
        ),
        (
            FightingStyle::Caster,
            MenuItem::with_info(
                FightingStyle::Caster.to_string(),
                "Caster: focuses, tomes, and arc conduits. Pros: spell amplification. Cons: fragile in melee.",
            ),
        ),
        (
            FightingStyle::Ranger,
            MenuItem::with_info(
                FightingStyle::Ranger.to_string(),
                "Ranger: bows or crossbows. Pros: range superiority. Cons: limited close-quarters answers.",
            ),
        ),
        (
            FightingStyle::Brawler,
            MenuItem::with_info(
                FightingStyle::Brawler.to_string(),
                "Brawler: fists, whips, and grapples. Pros: combos and counters. Cons: short reach.",
            ),
        ),
    ]
}

fn weapons_for(style: FightingStyle, tier: PhysicalTier) -> Vec<String> {
    weapon_definitions_for_style_with_tier(style, tier)
        .iter()
        .map(|definition| definition.weapon_name.to_string())
        .collect()
}

fn style_archetype(style: FightingStyle) -> WeaponArchetype {
    match style {
        FightingStyle::SwordsMan => WeaponArchetype::Swordsman,
        FightingStyle::DualBladeWelder => WeaponArchetype::DualBlade,
        FightingStyle::Tank => WeaponArchetype::Tank,
        FightingStyle::Caster => WeaponArchetype::Caster,
        FightingStyle::Ranger => WeaponArchetype::Ranger,
        FightingStyle::Brawler => WeaponArchetype::Brawler,
    }
}

fn families_for(style: FightingStyle) -> &'static [FamilyUnlock] {
    match style {
        FightingStyle::SwordsMan => SWORDSMAN_UNLOCKS,
        FightingStyle::DualBladeWelder => DUAL_UNLOCKS,
        FightingStyle::Tank => TANK_UNLOCKS,
        FightingStyle::Caster => CASTER_UNLOCKS,
        FightingStyle::Ranger => RANGER_UNLOCKS,
        FightingStyle::Brawler => BRAWLER_UNLOCKS,
    }
}

#[allow(dead_code)]
pub fn weapon_definitions_for_style(
    style: FightingStyle,
) -> Vec<&'static WeaponDefinition> {
    weapon_definitions_for_style_with_tier(style, PhysicalTier::Drifter)
}

pub fn weapon_definitions_for_style_with_tier(
    style: FightingStyle,
    tier: PhysicalTier,
) -> Vec<&'static WeaponDefinition> {
    let archetype = style_archetype(style);
    families_for(style)
        .iter()
        .filter(|unlock| unlock.is_unlocked(tier))
        .flat_map(|unlock| weapon_variants(unlock.family))
        .filter(|definition| definition.archetype == archetype)
        .collect()
}
