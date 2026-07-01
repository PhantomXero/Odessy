use super::fightstyle::{weapon_definitions_for_style_with_tier, FightingStyle, FightingStyleInfo};
use super::weapon::{weapon_catalog, ManaRequirement, WeaponDefinition, WeaponInfo, WeaponQuality, WeaponWeight};
use crate::physical::physical::{Physic, PhysicalInfo, PhysicalTier};
use crate::prompt::{select_from_menu, MenuItem};
use serde::{Deserialize, Serialize};
use std::fmt;

const COMBAT_GUIDE: &str = "Combat roles unlock talent trees. Pick one main class and optional sub roles to fine tune your kit.";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
        if prompt_yes_no("Add a supporting class?") {
            sub_classes.push(pick_class("Pick your support class: "));
        }

        let fighting_style = FightingStyleInfo::from_prompt(physical.tier());
        let weapon = pick_weapon(
            physical,
            fighting_style.primary_style(),
            allow_quality_choice,
        );
        let specialization = Self {
            main_class,
            sub_class: if sub_classes.is_empty() {
                None
            } else {
                Some(sub_classes)
            },
            main_fighting_style: Some(fighting_style),
            sub_fighting_style: None,
            main_weapon: weapon,
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
        if !weapon_allowed(&self.main_weapon, physical) {
            println!("Current weapon is too demanding for this physique. Reverting to Hands.");
            self.main_weapon = WeaponInfo::new();
        }
    }

    pub fn uses_caster_focus(&self) -> bool {
        let class_is_caster = matches!(
            self.main_class,
            Some(WarriorClass::Support | WarriorClass::Healer)
        );
        let style_is_caster = self
            .main_fighting_style
            .as_ref()
            .and_then(|style| style.primary_style())
            .map(|style| matches!(style, FightingStyle::Caster))
            .unwrap_or(false);
        class_is_caster || style_is_caster
    }

    pub fn hp_bonus(&self) -> u8 {
        match self.main_class {
            Some(WarriorClass::Tank) => 10,
            Some(WarriorClass::Vanguard) => 8,
            Some(WarriorClass::Brawler) => 7,
            Some(WarriorClass::Healer) => 6,
            Some(WarriorClass::Support) => 5,
            None => 0,
        }
    }

    pub fn weapon_badge(&self) -> String {
        self.main_weapon.badge()
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
    let options = vec![
        MenuItem::with_info("Yes", "Proceed with this choice."),
        MenuItem::with_info("No", "Skip for now."),
    ];
    let selection = select_from_menu(message, None, &options);
    println!("{} {}", message, selection.label);
    selection.index == 0
}

fn pick_class(prompt: &str) -> WarriorClass {
    let entries = class_entries();
    let menu: Vec<MenuItem> = entries.iter().map(|(_, item)| item.clone()).collect();
    let selection = select_from_menu(prompt, Some(COMBAT_GUIDE), &menu);
    let class = entries[selection.index].0;
    println!("{}{}", prompt, class);
    class
}

fn class_entries() -> Vec<(WarriorClass, MenuItem)> {
    vec![
        (
            WarriorClass::Support,
            MenuItem::with_info(
                "Support",
                "Battlefield coaches. Provide buffs, barriers, and tactical repositioning.",
            ),
        ),
        (
            WarriorClass::Tank,
            MenuItem::with_info(
                "Tank",
                "Frontline bulwark. High threat, heavy armor, and shield mastery.",
            ),
        ),
        (
            WarriorClass::Healer,
            MenuItem::with_info(
                "Healer",
                "Restorative arts. Cleanses, mends, and sustains mana efficiency.",
            ),
        ),
        (
            WarriorClass::Vanguard,
            MenuItem::with_info(
                "Vanguard",
                "Strike leaders. Aggressive tempo, gap closers, and crowd disruption.",
            ),
        ),
        (
            WarriorClass::Brawler,
            MenuItem::with_info(
                "Brawler",
                "Close-quarters bruiser. Chains combos, grapples, and improvised weapons.",
            ),
        ),
    ]
}

fn quality_entries(
    tier: PhysicalTier,
) -> Vec<(WeaponQuality, bool, MenuItem)> {
    let superior_unlocked = tier >= PhysicalTier::Vanguard;
    let master_unlocked = tier >= PhysicalTier::Titan;
    vec![
        (
            WeaponQuality::Standard,
            true,
            MenuItem::with_info(
                "Standard",
                "Balanced forging. Reliable upkeep and no special requirements.",
            ),
        ),
        (
            WeaponQuality::Superior,
            superior_unlocked,
            MenuItem::with_info(
                if superior_unlocked {
                    "Superior"
                } else {
                    "Superior [LOCKED – reach Vanguard]"
                },
                "Artisan gear reclaimed from elite armories. Needs Vanguard-tier physique.",
            ),
        ),
        (
            WeaponQuality::Masterwork,
            master_unlocked,
            MenuItem::with_info(
                if master_unlocked {
                    "Masterwork"
                } else {
                    "Masterwork [LOCKED – reach Titan]"
                },
                "Legendary prestige weapons hand-tuned for Titans and above.",
            ),
        ),
    ]
}

fn pick_weapon(
    physical: &PhysicalInfo,
    preferred_style: Option<FightingStyle>,
    allow_quality_choice: bool,
) -> WeaponInfo {
    let tier = physical.tier();
    let options: Vec<&WeaponDefinition> = match preferred_style {
        Some(style) => weapon_definitions_for_style_with_tier(style, tier),
        None => weapon_catalog().iter().collect(),
    };

    if options.is_empty() {
        println!(
            "No weapons unlocked for this style at tier {}. Staying with Hands until you grow stronger.",
            tier
        );
        return WeaponInfo::new();
    }

    let menu_items: Vec<MenuItem> = options
        .iter()
        .map(|definition| weapon_menu_entry(*definition))
        .collect();
    let guide = format!(
        "Tier {} | Speed {} | Strength {}",
        physical.tier(),
        physical.speed(),
        physical.strength()
    );
    println!(
        "Select a weapon (Tier {} | Speed {} | Strength {})",
        physical.tier(),
        physical.speed(),
        physical.strength()
    );
    loop {
        if let Some(style) = preferred_style {
            println!(
                "Weapons currently limited to the {} discipline (Tier {}). Improve your physique to unlock more families.",
                style,
                tier
            );
        }
        let selection = select_from_menu("Select a weapon", Some(&guide), &menu_items);
        let choice = options[selection.index];
        println!("Weapon choice: {}", choice);
        let requirement = weapon_requirement(choice, physical);
        if meets_requirement(&requirement, physical) {
            let quality = if allow_quality_choice {
                prompt_weapon_quality(physical.tier())
            } else {
                WeaponQuality::Standard
            };
            return WeaponInfo::from_definition_with_quality(choice, quality);
        }
        explain_requirement(&requirement, physical);
    }
}

fn weapon_allowed(weapon: &WeaponInfo, physical: &PhysicalInfo) -> bool {
    meets_requirement(&weapon_requirement(weapon.definition(), physical), physical)
}

fn prompt_weapon_quality(tier: PhysicalTier) -> WeaponQuality {
    let entries = quality_entries(tier);
    let menu: Vec<MenuItem> = entries.iter().map(|(_, _, item)| item.clone()).collect();
    println!("Select weapon quality (Physical tier: {})", tier);
    loop {
        let guide = format!("Physical tier: {}", tier);
        let selection = select_from_menu("Select weapon quality", Some(&guide), &menu);
        let (quality, unlocked, _) = &entries[selection.index];
        println!("Weapon Quality: {}", quality);
        if *unlocked {
            return *quality;
        }
        println!("Selection unavailable at the current tier.");
    }
}

struct WeaponRequirement {
    min_speed: u8,
    max_speed: Option<u8>,
    min_strength: u8,
    max_strength: Option<u8>,
    mana: ManaRequirement,
    allowed: bool,
    note: &'static str,
}

fn weapon_requirement(weapon: &WeaponDefinition, physical: &PhysicalInfo) -> WeaponRequirement {
    match weapon.weight {
        WeaponWeight::Light => WeaponRequirement {
            min_speed: 0,
            max_speed: None,
            min_strength: 0,
            max_strength: None,
            mana: weapon.mana_requirement(),
            allowed: true,
            note: "Light weapons are universally accessible.",
        },
        WeaponWeight::Medium => medium_requirement(physical, weapon),
        WeaponWeight::Heavy => heavy_requirement(physical, weapon),
    }
}

fn medium_requirement(physical: &PhysicalInfo, weapon: &WeaponDefinition) -> WeaponRequirement {
    match physical.physique() {
        Physic::Athletic => WeaponRequirement {
            min_speed: 12,
            max_speed: Some(80),
            min_strength: 18,
            max_strength: Some(80),
            mana: weapon.mana_requirement(),
            allowed: true,
            note: "Athletic builds are tuned for medium weapons.",
        },
        Physic::Lean => WeaponRequirement {
            min_speed: 20,
            max_speed: None,
            min_strength: 14,
            max_strength: Some(55),
            mana: weapon.mana_requirement(),
            allowed: true,
            note: "Lean fighters need exceptional speed but must stay within manageable force windows.",
        },
        Physic::Muscular => WeaponRequirement {
            min_speed: 15,
            max_speed: Some(70),
            min_strength: 22,
            max_strength: Some(92),
            mana: weapon.mana_requirement(),
            allowed: true,
            note: "Muscular frames can swing medium gear if they keep their stance light.",
        },
    }
}

fn heavy_requirement(physical: &PhysicalInfo, weapon: &WeaponDefinition) -> WeaponRequirement {
    if weapon.is_prestige && physical.tier() < PhysicalTier::Titan {
        return WeaponRequirement {
            min_speed: 18,
            max_speed: None,
            min_strength: 36,
            max_strength: None,
            mana: weapon.mana_requirement(),
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
            mana: weapon.mana_requirement(),
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
                    mana: weapon.mana_requirement(),
                    allowed: true,
                    note: "Only Titan-tier athletic builds can stretch into heavy weaponry.",
                }
            } else {
                WeaponRequirement {
                    min_speed: 16,
                    max_speed: None,
                    min_strength: 32,
                    max_strength: None,
                    mana: weapon.mana_requirement(),
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
            mana: weapon.mana_requirement(),
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
    let mana = physical.mana_thresholds();
    if mana.pressure < req.mana.pressure {
        return false;
    }
    if mana.recovery < req.mana.recovery {
        return false;
    }
    true
}

fn explain_requirement(req: &WeaponRequirement, physical: &PhysicalInfo) {
    println!("{}", req.note);
    if req.allowed {
        let speed_clause = format_requirement("Speed", req.min_speed, req.max_speed);
        let strength_clause = format_requirement("Strength", req.min_strength, req.max_strength);
        println!(
            "Needs {}, {} and mana pressure >= {} / recovery >= {}. Current Speed {} / Strength {} / Mana {}|{}.",
            speed_clause,
            strength_clause,
            req.mana.pressure,
            req.mana.recovery,
            physical.speed(),
            physical.strength(),
            physical.mana_thresholds().pressure,
            physical.mana_thresholds().recovery
        );
    }
}

fn format_requirement(label: &str, min: u8, max: Option<u8>) -> String {
    match max {
        Some(max) => format!("{} between {} and {}", label, min, max),
        None => format!("{} >= {}", label, min),
    }
}

fn weapon_menu_entry(weapon: &WeaponDefinition) -> MenuItem {
    let mana = weapon.mana_requirement();
    let stats = weapon.discipline().describe();
    let info = format!(
        "Damage {:.1} | Mana P{} / R{} | {}\n{}",
        weapon.base_damage,
        mana.pressure,
        mana.recovery,
        stats,
        weapon.flavor
    );
    MenuItem::with_info(
        format!("{} [{} | {:?}]", weapon, weapon.family, weapon.weight),
        info,
    )
}
