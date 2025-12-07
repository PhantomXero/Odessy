use super::fightstyle::FightingStyleInfo;
use super::weapon::{Weapon, WeaponInfo, WeaponQuality, WeaponWeight};
use crate::physical::physical::{Physic, PhysicalInfo, PhysicalTier};
use el_roi::read_int;
use serde::{Deserialize, Serialize};
use std::fmt;

const COMBAT_GUIDE: &str = "Combat roles unlock talent trees. Pick one main class and optional sub roles to fine tune your kit.";

#[derive(Debug, Clone, Serialize, Deserialize)]
enum WarriorClass {
    Support,
    Tank,
    Healer,
    Vanguard,
    Brawler,
}

impl fmt::Display for WarriorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WarriorClass::Support => "Support",
            WarriorClass::Tank => "Tank",
            WarriorClass::Healer => "Healer",
            WarriorClass::Vanguard => "Vanguard",
            WarriorClass::Brawler => "Brawler",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum WarriorRank {
    UnRanked,
    Novice,
    Amateur,
    Intermediate,
    Bronze,
    Sliver,
    Gold,
}

impl fmt::Display for WarriorRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WarriorRank::UnRanked => "Unranked",
            WarriorRank::Novice => "Novice",
            WarriorRank::Amateur => "Amateur",
            WarriorRank::Intermediate => "Intermediate",
            WarriorRank::Bronze => "Bronze",
            WarriorRank::Sliver => "Silver",
            WarriorRank::Gold => "Gold",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarriorInfo {
    main_class: Option<WarriorClass>,
    sub_class: Option<Vec<WarriorClass>>,
    main_fighting_style: Option<FightingStyleInfo>,
    sub_fighting_style: Option<Vec<FightingStyleInfo>>,
    main_weapon: WeaponInfo,
    rank: WarriorRank,
    #[serde(default)]
    respec_token: bool,
}

impl WarriorInfo {
    pub fn new() -> Self {
        Self {
            main_class: None,
            sub_class: None,
            main_fighting_style: None,
            sub_fighting_style: None,
            main_weapon: WeaponInfo::new(),
            rank: WarriorRank::UnRanked,
            respec_token: false,
        }
    }

    pub fn from_prompt(physical: &PhysicalInfo) -> Self {
        Self::from_prompt_with_options(physical, false)
    }

    fn from_prompt_with_options(physical: &PhysicalInfo, allow_quality_choice: bool) -> Self {
        println!("--- Combat Archetype ---");
        println!("{}", COMBAT_GUIDE);
        println!(
            "Physical Tier: {} (Speed {} | Strength {})",
            physical.tier(),
            physical.speed(),
            physical.strength()
        );
        println!("Higher tiers unlock heavier gear, prestige weapons, and quality upgrades.");
        let main_class = Some(pick_class("Choose your main class: "));
        let mut sub_classes = Vec::new();
        if prompt_yes_no("Add a supporting class? (1 Yes / 2 No)") {
            sub_classes.push(pick_class("Pick your support class: "));
        }

        let specialization = Self {
            main_class,
            sub_class: if sub_classes.is_empty() {
                None
            } else {
                Some(sub_classes)
            },
            main_fighting_style: Some(FightingStyleInfo::from_prompt()),
            sub_fighting_style: None,
            main_weapon: pick_weapon(physical, allow_quality_choice),
            rank: WarriorRank::UnRanked,
            respec_token: false,
        };
        specialization
    }

    pub fn level_up(&mut self) {
        self.rank = match self.rank {
            WarriorRank::UnRanked => WarriorRank::Novice,
            WarriorRank::Novice => WarriorRank::Amateur,
            WarriorRank::Amateur => WarriorRank::Intermediate,
            WarriorRank::Intermediate => WarriorRank::Bronze,
            WarriorRank::Bronze => WarriorRank::Sliver,
            WarriorRank::Sliver => WarriorRank::Gold,
            WarriorRank::Gold => WarriorRank::Gold,
        };
    }

    pub fn edit(&mut self, physical: &PhysicalInfo) {
        if !self.respec_token {
            println!("Combat specialization changes are locked behind story events.");
            println!(
                "Earn a respec token before reconfiguring class, fighting style, or weapon quality."
            );
            return;
        }
        let mut updated = WarriorInfo::from_prompt_with_options(physical, true);
        updated.rank = self.rank;
        *self = updated;
    }

    pub fn grant_respec_token(&mut self) {
        self.respec_token = true;
    }

    pub fn sync_with_physical(&mut self, physical: &PhysicalInfo) {
        if !weapon_allowed(*self.main_weapon.class(), physical) {
            println!("Current weapon is too demanding for this physique. Reverting to Hands.");
            self.main_weapon = WeaponInfo::new();
        }
    }
}

impl Default for WarriorInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WarriorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let main_class = self
            .main_class
            .as_ref()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "Unassigned".to_string());
        let sub_classes = self
            .sub_class
            .as_ref()
            .map(|list| {
                list.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "None".to_string());
        let fighting_style = self
            .main_fighting_style
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Style: Unassigned | Weapons: None".to_string());
        write!(
            f,
            "Rank: {}\nMain Class: {}\nSub Classes: {}\n{}\nWeapon: {}",
            self.rank, main_class, sub_classes, fighting_style, self.main_weapon
        )
    }
}

fn prompt_yes_no(message: &str) -> bool {
    println!("{}", message);
    matches!(read_int("Selection: "), 1)
}

fn pick_class(prompt: &str) -> WarriorClass {
    println!("{}", prompt);
    println!("1. Support\n2. Tank\n3. Healer\n4. Vanguard\n5. Brawler");
    match read_int("Selection: ") {
        1 => WarriorClass::Support,
        2 => WarriorClass::Tank,
        3 => WarriorClass::Healer,
        4 => WarriorClass::Vanguard,
        5 => WarriorClass::Brawler,
        _ => WarriorClass::Vanguard,
    }
}

fn pick_weapon(physical: &PhysicalInfo, allow_quality_choice: bool) -> WeaponInfo {
    loop {
        println!(
            "Select a weapon (Tier {} | Speed {} | Strength {})",
            physical.tier(),
            physical.speed(),
            physical.strength()
        );
        let options = WeaponInfo::new_weapons();
        for (idx, weapon) in options.iter().enumerate() {
            println!("{}. {} ({:?})", idx + 1, weapon, weapon.weight());
        }
        let selection = read_int("Selection: ") - 1;
        if let Some(choice) = options.get(selection as usize) {
            let requirement = weapon_requirement(*choice, physical);
            if meets_requirement(&requirement, physical) {
                let quality = if allow_quality_choice {
                    prompt_weapon_quality(physical.tier())
                } else {
                    WeaponQuality::Standard
                };
                return WeaponInfo::from_weapon_with_quality(*choice, quality);
            }
            explain_requirement(&requirement, physical);
        } else {
            println!("Invalid weapon selection.");
        }
    }
}

fn weapon_allowed(weapon: Weapon, physical: &PhysicalInfo) -> bool {
    meets_requirement(&weapon_requirement(weapon, physical), physical)
}

fn prompt_weapon_quality(tier: PhysicalTier) -> WeaponQuality {
    loop {
        println!("Select weapon quality (Physical tier: {}):", tier);
        println!("1. Standard (balanced)");
        if tier >= PhysicalTier::Vanguard {
            println!("2. Superior (artisan forged)");
        } else {
            println!("2. Superior [LOCKED – reach Vanguard tier]");
        }
        if tier >= PhysicalTier::Titan {
            println!("3. Masterwork (legendary prestige)");
        } else {
            println!("3. Masterwork [LOCKED – reach Titan tier]");
        }
        println!("(Crude gear only appears via loot/crafting mishaps.)");
        match read_int("Selection: ") {
            1 => return WeaponQuality::Standard,
            2 if tier >= PhysicalTier::Vanguard => return WeaponQuality::Superior,
            3 if tier >= PhysicalTier::Titan => return WeaponQuality::Masterwork,
            _ => println!("Selection unavailable at the current tier."),
        }
    }
}

struct WeaponRequirement {
    min_speed: u8,
    max_speed: Option<u8>,
    min_strength: u8,
    max_strength: Option<u8>,
    allowed: bool,
    note: &'static str,
}

fn weapon_requirement(weapon: Weapon, physical: &PhysicalInfo) -> WeaponRequirement {
    match weapon.weight() {
        WeaponWeight::Light => WeaponRequirement {
            min_speed: 0,
            max_speed: None,
            min_strength: 0,
            max_strength: None,
            allowed: true,
            note: "Light weapons are universally accessible.",
        },
        WeaponWeight::Medium => medium_requirement(physical),
        WeaponWeight::Heavy => heavy_requirement(physical, weapon),
    }
}

fn medium_requirement(physical: &PhysicalInfo) -> WeaponRequirement {
    match physical.physique() {
        Physic::Athletic => WeaponRequirement {
            min_speed: 12,
            max_speed: Some(80),
            min_strength: 18,
            max_strength: Some(80),
            allowed: true,
            note: "Athletic builds are tuned for medium weapons.",
        },
        Physic::Lean => WeaponRequirement {
            min_speed: 20,
            max_speed: None,
            min_strength: 14,
            max_strength: Some(55),
            allowed: true,
            note: "Lean fighters need exceptional speed but must stay within manageable force windows.",
        },
        Physic::Muscular => WeaponRequirement {
            min_speed: 15,
            max_speed: Some(70),
            min_strength: 22,
            max_strength: Some(92),
            allowed: true,
            note: "Muscular frames can swing medium gear if they keep their stance light.",
        },
    }
}

fn heavy_requirement(physical: &PhysicalInfo, weapon: Weapon) -> WeaponRequirement {
    if weapon.is_prestige() && physical.tier() < PhysicalTier::Titan {
        return WeaponRequirement {
            min_speed: 18,
            max_speed: None,
            min_strength: 36,
            max_strength: None,
            allowed: false,
            note: "Prestige weapons demand Titan-tier physique before they resonate.",
        };
    }
    match physical.physique() {
        Physic::Muscular => WeaponRequirement {
            min_speed: 12,
            max_speed: None,
            min_strength: 34,
            max_strength: None,
            allowed: true,
            note: "Muscular bodies are built for heavy weapons.",
        },
        Physic::Athletic => {
            let tier = physical.tier();
            if tier >= PhysicalTier::Titan {
                WeaponRequirement {
                    min_speed: 16,
                    max_speed: None,
                    min_strength: 32,
                    max_strength: None,
                    allowed: true,
                    note: "Only Titan-tier athletic builds can stretch into heavy weaponry.",
                }
            } else {
                WeaponRequirement {
                    min_speed: 16,
                    max_speed: None,
                    min_strength: 32,
                    max_strength: None,
                    allowed: false,
                    note: "Reach Titan tier before attempting heavy weapons with an athletic build.",
                }
            }
        }
        Physic::Lean => WeaponRequirement {
            min_speed: 0,
            max_speed: None,
            min_strength: 0,
            max_strength: None,
            allowed: false,
            note: "Lean frames cannot support heavy weapons.",
        },
    }
}

fn meets_requirement(req: &WeaponRequirement, physical: &PhysicalInfo) -> bool {
    if !req.allowed {
        return false;
    }
    if physical.speed() < req.min_speed {
        return false;
    }
    if let Some(max) = req.max_speed {
        if physical.speed() > max {
            return false;
        }
    }
    if physical.strength() < req.min_strength {
        return false;
    }
    if let Some(max) = req.max_strength {
        if physical.strength() > max {
            return false;
        }
    }
    true
}

fn explain_requirement(req: &WeaponRequirement, physical: &PhysicalInfo) {
    println!("{}", req.note);
    if req.allowed {
        let speed_clause = format_requirement("Speed", req.min_speed, req.max_speed);
        let strength_clause = format_requirement("Strength", req.min_strength, req.max_strength);
        println!(
            "Needs {} and {}. Current Speed {} / Strength {}.",
            speed_clause,
            strength_clause,
            physical.speed(),
            physical.strength()
        );
    }
}

fn format_requirement(label: &str, min: u8, max: Option<u8>) -> String {
    match max {
        Some(max) => format!("{} between {} and {}", label, min, max),
        None => format!("{} >= {}", label, min),
    }
}
