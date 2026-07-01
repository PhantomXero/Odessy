use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponArchetype {
    Swordsman,
    DualBlade,
    Tank,
    Caster,
    Ranger,
    Brawler,
}

impl fmt::Display for WeaponArchetype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WeaponArchetype::Swordsman => "Swordsman",
            WeaponArchetype::DualBlade => "Dual Blade",
            WeaponArchetype::Tank => "Tank",
            WeaponArchetype::Caster => "Caster",
            WeaponArchetype::Ranger => "Ranger",
            WeaponArchetype::Brawler => "Brawler",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WeaponBlueprint {
    Hands,
    Gauntlets,
    Dagger,
    Whip,
    MeteorGloves,
    TonfaPair,
    RopeDart,
    KatarPunchers,
    ShortSword,
    Saber,
    LongSword,
    Claymore,
    Gladius,
    Falchion,
    BastardSword,
    Katana,
    DualShortSword,
    TwinDaggers,
    DualLongSword,
    MirrorSabers,
    SpiralKnives,
    SunAndMoon,
    EclipseBlades,
    Axe,
    Hammer,
    WarMaul,
    TowerShield,
    BulwarkShield,
    SiegeAxe,
    RunicHammer,
    ThunderMaul,
    AegisShield,
    ChannelingStaff,
    FocusingWand,
    Spellbook,
    ArcaneOrb,
    SingingStaff,
    ObsidianWand,
    LivingCodex,
    StellarFocus,
    FrostTome,
    ShortBow,
    LongBow,
    Crossbow,
    CompositeBow,
    HawkLongbow,
    RepeatingCrossbow,
    GaleShortbow,
    SiegeCrossbow,
    SpiritLongbow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponFamily {
    Unarmed,
    Gauntlet,
    Dagger,
    Whip,
    ShortBlade,
    LongBlade,
    DualShortBlade,
    DualLongBlade,
    DualDagger,
    Axe,
    Hammer,
    Maul,
    Shield,
    Staff,
    Wand,
    Codex,
    Focus,
    ShortBow,
    LongBow,
    Crossbow,
}

impl fmt::Display for WeaponFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

impl WeaponFamily {
    fn label(self) -> &'static str {
        match self {
            WeaponFamily::Unarmed => "Unarmed",
            WeaponFamily::Gauntlet => "Gauntlet",
            WeaponFamily::Dagger => "Dagger",
            WeaponFamily::Whip => "Whip",
            WeaponFamily::ShortBlade => "Short Blade",
            WeaponFamily::LongBlade => "Long Blade",
            WeaponFamily::DualShortBlade => "Dual Short Blade",
            WeaponFamily::DualLongBlade => "Dual Long Blade",
            WeaponFamily::DualDagger => "Dual Daggers",
            WeaponFamily::Axe => "Axe",
            WeaponFamily::Hammer => "Hammer",
            WeaponFamily::Maul => "Maul",
            WeaponFamily::Shield => "Shield",
            WeaponFamily::Staff => "Staff",
            WeaponFamily::Wand => "Wand",
            WeaponFamily::Codex => "Spellbook",
            WeaponFamily::Focus => "Arcane Focus",
            WeaponFamily::ShortBow => "Short Bow",
            WeaponFamily::LongBow => "Long Bow",
            WeaponFamily::Crossbow => "Crossbow",
        }
    }

    fn emoji(self) -> &'static str {
        match self {
            WeaponFamily::Unarmed => "👐",
            WeaponFamily::Gauntlet => "🥊",
            WeaponFamily::Dagger => "🗡️",
            WeaponFamily::Whip => "🪢",
            WeaponFamily::ShortBlade => "🗡️",
            WeaponFamily::LongBlade => "⚔️",
            WeaponFamily::DualShortBlade => "⚔️",
            WeaponFamily::DualLongBlade => "⚔️",
            WeaponFamily::DualDagger => "🗡️",
            WeaponFamily::Axe => "🪓",
            WeaponFamily::Hammer => "🔨",
            WeaponFamily::Maul => "⚒️",
            WeaponFamily::Shield => "🛡️",
            WeaponFamily::Staff => "🦯",
            WeaponFamily::Wand => "🪄",
            WeaponFamily::Codex => "📖",
            WeaponFamily::Focus => "🔮",
            WeaponFamily::ShortBow => "🏹",
            WeaponFamily::LongBow => "🏹",
            WeaponFamily::Crossbow => "🎯",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeaponWeight {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeaponQuality {
    Crude,
    Standard,
    Superior,
    Masterwork,
}

impl Default for WeaponQuality {
    fn default() -> Self {
        WeaponQuality::Standard
    }
}

impl WeaponQuality {
    pub fn durability_bonus(self) -> f64 {
        match self {
            WeaponQuality::Crude => -8.0,
            WeaponQuality::Standard => 0.0,
            WeaponQuality::Superior => 12.0,
            WeaponQuality::Masterwork => 20.0,
        }
    }

    pub fn damage_bonus(self) -> f64 {
        match self {
            WeaponQuality::Crude => -2.0,
            WeaponQuality::Standard => 0.0,
            WeaponQuality::Superior => 3.0,
            WeaponQuality::Masterwork => 5.0,
        }
    }

    pub fn magic_resistance(self) -> f32 {
        match self {
            WeaponQuality::Crude => 0.0,
            WeaponQuality::Standard => 0.05,
            WeaponQuality::Superior => 0.12,
            WeaponQuality::Masterwork => 0.2,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ManaRequirement {
    pub pressure: u8,
    pub recovery: u8,
}

impl ManaRequirement {
    pub const fn new(pressure: u8, recovery: u8) -> Self {
        Self { pressure, recovery }
    }

    pub const fn none() -> Self {
        Self::new(0, 0)
    }

    pub const fn low() -> Self {
        Self::new(3, 2)
    }

    pub const fn custom(pressure: u8, recovery: u8) -> Self {
        Self::new(pressure, recovery)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlSpread {
    pub dominant: u8,
    pub offhand: u8,
}

impl ControlSpread {
    pub const fn new(dominant: u8, offhand: u8) -> Self {
        Self { dominant, offhand }
    }

    pub const fn total(self) -> u8 {
        self.dominant + self.offhand
    }

    pub fn describe(&self) -> String {
        format!("Control {} (dom) / {} (off)", self.dominant, self.offhand)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct WeaponDiscipline {
    pub control: ControlSpread,
    pub power: u8,
    pub range: u8,
}

impl WeaponDiscipline {
    pub const TOTAL_STARTER_POINTS: u8 = 15;

    pub const fn new(control: ControlSpread, power: u8, range: u8) -> Self {
        Self {
            control,
            power,
            range,
        }
    }

    pub const fn total_points(&self) -> u8 {
        self.control.total() + self.power + self.range
    }

    pub fn describe(&self) -> String {
        format!(
            "{} | Power {} | Range {}",
            self.control.describe(),
            self.power,
            self.range
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponDefinition {
    pub blueprint: WeaponBlueprint,
    pub weapon_name: &'static str,
    pub archetype: WeaponArchetype,
    pub family: WeaponFamily,
    pub weight: WeaponWeight,
    pub base_damage: f64,
    pub base_durability: f64,
    pub is_prestige: bool,
    pub mana: ManaRequirement,
    pub discipline: WeaponDiscipline,
    pub flavor: &'static str,
}

impl WeaponDefinition {
    pub const fn new(
        blueprint: WeaponBlueprint,
        weapon_name: &'static str,
        archetype: WeaponArchetype,
        family: WeaponFamily,
        weight: WeaponWeight,
        base_damage: f64,
        base_durability: f64,
        is_prestige: bool,
        mana: ManaRequirement,
        discipline: WeaponDiscipline,
        flavor: &'static str,
    ) -> Self {
        Self {
            blueprint,
            weapon_name,
            archetype,
            family,
            weight,
            base_damage,
            base_durability,
            is_prestige,
            mana,
            discipline,
            flavor,
        }
    }

    pub fn mana_requirement(&self) -> ManaRequirement {
        self.mana
    }

    pub fn blueprint(&self) -> WeaponBlueprint {
        self.blueprint
    }

    pub fn discipline(&self) -> WeaponDiscipline {
        self.discipline
    }
}

impl fmt::Display for WeaponDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.weapon_name)
    }
}

pub struct WeaponDatabase {
    records: &'static [WeaponDefinition],
}

impl WeaponDatabase {
    pub const fn new(records: &'static [WeaponDefinition]) -> Self {
        Self { records }
    }

    pub fn all(&self) -> &'static [WeaponDefinition] {
        self.records
    }

    pub fn by_family(&self, family: WeaponFamily) -> Vec<&'static WeaponDefinition> {
        self.records
            .iter()
            .filter(|definition| definition.family == family)
            .collect()
    }

    pub fn by_blueprint(
        &self,
        blueprint: WeaponBlueprint,
    ) -> Option<&'static WeaponDefinition> {
        self.records
            .iter()
            .find(|definition| definition.blueprint == blueprint)
    }

    pub fn by_name(&self, name: &str) -> Option<&'static WeaponDefinition> {
        self.records
            .iter()
            .find(|definition| definition.weapon_name.eq_ignore_ascii_case(name))
    }
}


const WEAPON_LIBRARY: &[WeaponDefinition] = &[
    // Brawler catalog
    WeaponDefinition::new(
        WeaponBlueprint::Hands,
        "Hands",
        WeaponArchetype::Brawler,
        WeaponFamily::Unarmed,
        WeaponWeight::Light,
        8.0,
        22.0,
        false,
        ManaRequirement::none(),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 4, 3),
        "Baseline martial arts. Grapples, throws, and improvised counters.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Gauntlets,
        "Gauntlets",
        WeaponArchetype::Brawler,
        WeaponFamily::Gauntlet,
        WeaponWeight::Medium,
        18.0,
        24.0,
        false,
        ManaRequirement::low(),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 6, 3),
        "Brutal fist weapons that add weight to every punch.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Dagger,
        "Dagger",
        WeaponArchetype::Brawler,
        WeaponFamily::Dagger,
        WeaponWeight::Light,
        14.0,
        22.0,
        false,
        ManaRequirement::none(),
        WeaponDiscipline::new(ControlSpread::new(4, 4), 3, 4),
        "Light assassin blade. Thrives on speed and precision strikes.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Whip,
        "Whip",
        WeaponArchetype::Brawler,
        WeaponFamily::Whip,
        WeaponWeight::Light,
        11.0,
        22.0,
        false,
        ManaRequirement::low(),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 2, 5),
        "Crowd-control tether. Disarms, entangles, and punishes spacing mistakes.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::MeteorGloves,
        "Meteor Gloves",
        WeaponArchetype::Brawler,
        WeaponFamily::Gauntlet,
        WeaponWeight::Medium,
        20.0,
        25.0,
        false,
        ManaRequirement::custom(8, 6),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 6, 3),
        "Rune-lined gauntlets that detonate on impact.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::TonfaPair,
        "Tonfa Pair",
        WeaponArchetype::Brawler,
        WeaponFamily::Gauntlet,
        WeaponWeight::Medium,
        18.0,
        24.0,
        false,
        ManaRequirement::custom(6, 4),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 4, 3),
        "Arm-guard batons that pivot between defense and flurries.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::RopeDart,
        "Rope Dart",
        WeaponArchetype::Brawler,
        WeaponFamily::Whip,
        WeaponWeight::Light,
        17.0,
        21.0,
        false,
        ManaRequirement::custom(10, 8),
        WeaponDiscipline::new(ControlSpread::new(6, 2), 3, 4),
        "Chain-and-dart hybrid for harassing pulls and punctures.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::KatarPunchers,
        "Katar Punchers",
        WeaponArchetype::Brawler,
        WeaponFamily::Dagger,
        WeaponWeight::Light,
        21.0,
        23.0,
        false,
        ManaRequirement::low(),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 5, 3),
        "Punch daggers tuned for brutal close-range finishers.",
    ),
    // Swordsman catalog
    WeaponDefinition::new(
        WeaponBlueprint::ShortSword,
        "Short Sword",
        WeaponArchetype::Swordsman,
        WeaponFamily::ShortBlade,
        WeaponWeight::Medium,
        20.0,
        24.0,
        false,
        ManaRequirement::custom(6, 5),
        WeaponDiscipline::new(ControlSpread::new(5, 2), 5, 3),
        "Reliable infantry blade with a balanced guard.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Saber,
        "Saber",
        WeaponArchetype::Swordsman,
        WeaponFamily::ShortBlade,
        WeaponWeight::Medium,
        22.0,
        24.0,
        false,
        ManaRequirement::custom(6, 5),
        WeaponDiscipline::new(ControlSpread::new(5, 2), 4, 4),
        "Curved dueling blade built for sweeping slashes.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::LongSword,
        "Long Sword",
        WeaponArchetype::Swordsman,
        WeaponFamily::LongBlade,
        WeaponWeight::Heavy,
        30.0,
        26.0,
        false,
        ManaRequirement::custom(10, 8),
        WeaponDiscipline::new(ControlSpread::new(5, 2), 6, 2),
        "Knightly mainstay that mixes reach with stability.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Claymore,
        "Claymore",
        WeaponArchetype::Swordsman,
        WeaponFamily::LongBlade,
        WeaponWeight::Heavy,
        32.0,
        26.0,
        true,
        ManaRequirement::custom(12, 9),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 8, 1),
        "Massive two-hander that cleaves lines of infantry.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Gladius,
        "Gladius",
        WeaponArchetype::Swordsman,
        WeaponFamily::ShortBlade,
        WeaponWeight::Medium,
        21.0,
        24.0,
        false,
        ManaRequirement::custom(5, 4),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 5, 4),
        "Legionnaire sword that excels at shield-line thrusts.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Falchion,
        "Falchion",
        WeaponArchetype::Swordsman,
        WeaponFamily::ShortBlade,
        WeaponWeight::Medium,
        23.0,
        24.0,
        false,
        ManaRequirement::custom(7, 5),
        WeaponDiscipline::new(ControlSpread::new(5, 2), 5, 3),
        "Single-edge cutter that trades finesse for high-tempo swings.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::BastardSword,
        "Bastard Sword",
        WeaponArchetype::Swordsman,
        WeaponFamily::LongBlade,
        WeaponWeight::Heavy,
        31.0,
        26.0,
        false,
        ManaRequirement::custom(12, 8),
        WeaponDiscipline::new(ControlSpread::new(5, 2), 6, 2),
        "Versatile greatsword for half-swording or full-power cleaves.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Katana,
        "Katana",
        WeaponArchetype::Swordsman,
        WeaponFamily::LongBlade,
        WeaponWeight::Medium,
        28.0,
        25.0,
        false,
        ManaRequirement::custom(10, 7),
        WeaponDiscipline::new(ControlSpread::new(5, 2), 5, 3),
        "Draw-cut blade rewarding precise timing and stance work.",
    ),
    // Dual Blade catalog
    WeaponDefinition::new(
        WeaponBlueprint::DualShortSword,
        "Dual Short Sword",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualShortBlade,
        WeaponWeight::Medium,
        24.0,
        24.0,
        false,
        ManaRequirement::custom(6, 5),
        WeaponDiscipline::new(ControlSpread::new(4, 4), 5, 2),
        "Ambidextrous offense that floods foes with cuts.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::TwinDaggers,
        "Twin Daggers",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualDagger,
        WeaponWeight::Light,
        19.0,
        22.0,
        false,
        ManaRequirement::none(),
        WeaponDiscipline::new(ControlSpread::new(4, 4), 4, 3),
        "Silent killers. Pair of daggers for relentless close pressure.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::DualLongSword,
        "Dual Long Sword",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualLongBlade,
        WeaponWeight::Heavy,
        34.0,
        26.0,
        true,
        ManaRequirement::custom(16, 10),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 6, 1),
        "Prestige pairing that demands mastery to stay balanced.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::MirrorSabers,
        "Mirror Sabers",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualShortBlade,
        WeaponWeight::Medium,
        26.0,
        24.0,
        false,
        ManaRequirement::custom(10, 6),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 4, 3),
        "Synchronized scimitars designed for mirrored counters.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::SpiralKnives,
        "Spiral Knives",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualDagger,
        WeaponWeight::Light,
        20.0,
        22.0,
        false,
        ManaRequirement::custom(6, 4),
        WeaponDiscipline::new(ControlSpread::new(4, 4), 3, 4),
        "Curved daggers that spin through grapples and disarms.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::SunAndMoon,
        "Sun & Moon Blades",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualLongBlade,
        WeaponWeight::Heavy,
        33.0,
        25.0,
        true,
        ManaRequirement::custom(14, 9),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 5, 2),
        "Sacred twin blades channeling solar and lunar rites.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::EclipseBlades,
        "Eclipse Blades",
        WeaponArchetype::DualBlade,
        WeaponFamily::DualLongBlade,
        WeaponWeight::Heavy,
        32.0,
        25.0,
        true,
        ManaRequirement::custom(15, 10),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 6, 1),
        "Shadow-charged pair meant for decisive execution strings.",
    ),
    // Tank catalog
    WeaponDefinition::new(
        WeaponBlueprint::Axe,
        "Axe",
        WeaponArchetype::Tank,
        WeaponFamily::Axe,
        WeaponWeight::Heavy,
        32.0,
        26.0,
        false,
        ManaRequirement::custom(10, 6),
        WeaponDiscipline::new(ControlSpread::new(4, 1), 8, 2),
        "Chopping edge tuned for armor-rending blows.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Hammer,
        "Hammer",
        WeaponArchetype::Tank,
        WeaponFamily::Hammer,
        WeaponWeight::Heavy,
        34.0,
        26.0,
        true,
        ManaRequirement::custom(12, 6),
        WeaponDiscipline::new(ControlSpread::new(3, 1), 9, 2),
        "Impact weapon that caves shields and staggers titans.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::WarMaul,
        "War Maul",
        WeaponArchetype::Tank,
        WeaponFamily::Maul,
        WeaponWeight::Heavy,
        36.0,
        26.0,
        true,
        ManaRequirement::custom(16, 8),
        WeaponDiscipline::new(ControlSpread::new(3, 1), 10, 1),
        "Siege-class maul. Launches shockwaves with every slam.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::TowerShield,
        "Tower Shield",
        WeaponArchetype::Tank,
        WeaponFamily::Shield,
        WeaponWeight::Heavy,
        16.0,
        26.0,
        false,
        ManaRequirement::custom(8, 4),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 3, 5),
        "Full-body shield for fortress tactics and line holds.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::BulwarkShield,
        "Bulwark Shield",
        WeaponArchetype::Tank,
        WeaponFamily::Shield,
        WeaponWeight::Heavy,
        20.0,
        26.0,
        true,
        ManaRequirement::custom(12, 6),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 4, 4),
        "Prestige shield with embedded runes for retaliatory bumps.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::SiegeAxe,
        "Siege Axe",
        WeaponArchetype::Tank,
        WeaponFamily::Axe,
        WeaponWeight::Heavy,
        34.0,
        26.0,
        false,
        ManaRequirement::custom(12, 7),
        WeaponDiscipline::new(ControlSpread::new(4, 1), 9, 1),
        "Broad-bladed axe used to break wagons and bulwarks alike.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::RunicHammer,
        "Runic Hammer",
        WeaponArchetype::Tank,
        WeaponFamily::Hammer,
        WeaponWeight::Heavy,
        35.0,
        26.0,
        true,
        ManaRequirement::custom(14, 8),
        WeaponDiscipline::new(ControlSpread::new(3, 1), 9, 2),
        "Arc-scripted hammer that channels tremor wards on impact.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::ThunderMaul,
        "Thunder Maul",
        WeaponArchetype::Tank,
        WeaponFamily::Maul,
        WeaponWeight::Heavy,
        37.0,
        26.0,
        true,
        ManaRequirement::custom(16, 10),
        WeaponDiscipline::new(ControlSpread::new(3, 1), 10, 1),
        "Storm-canister maul that releases concussive bursts.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::AegisShield,
        "Aegis Shield",
        WeaponArchetype::Tank,
        WeaponFamily::Shield,
        WeaponWeight::Heavy,
        18.0,
        28.0,
        false,
        ManaRequirement::custom(10, 6),
        WeaponDiscipline::new(ControlSpread::new(4, 4), 3, 4),
        "Angled wall-shield offering superior coverage and bash potential.",
    ),
    // Caster catalog
    WeaponDefinition::new(
        WeaponBlueprint::ChannelingStaff,
        "Channeling Staff",
        WeaponArchetype::Caster,
        WeaponFamily::Staff,
        WeaponWeight::Medium,
        18.0,
        24.0,
        false,
        ManaRequirement::custom(28, 24),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 4, 4),
        "Arcanists' staff that stabilizes long-form spellwork.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::FocusingWand,
        "Focusing Wand",
        WeaponArchetype::Caster,
        WeaponFamily::Wand,
        WeaponWeight::Light,
        14.0,
        22.0,
        false,
        ManaRequirement::custom(24, 26),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 3, 5),
        "Precision wand for rapid-fire spell bursts.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Spellbook,
        "Spellbook",
        WeaponArchetype::Caster,
        WeaponFamily::Codex,
        WeaponWeight::Light,
        12.0,
        22.0,
        false,
        ManaRequirement::custom(20, 28),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 2, 5),
        "Glyph-weaving grimoire that amplifies ritual casting.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::ArcaneOrb,
        "Arcane Orb",
        WeaponArchetype::Caster,
        WeaponFamily::Focus,
        WeaponWeight::Light,
        20.0,
        22.0,
        true,
        ManaRequirement::custom(32, 18),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 4, 4),
        "Hovering focus used to shape beams and sigils mid-air.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::SingingStaff,
        "Singing Staff",
        WeaponArchetype::Caster,
        WeaponFamily::Staff,
        WeaponWeight::Medium,
        19.0,
        24.0,
        false,
        ManaRequirement::custom(26, 20),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 4, 4),
        "Harmonic conduit that steadies channeled melodies.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::ObsidianWand,
        "Obsidian Wand",
        WeaponArchetype::Caster,
        WeaponFamily::Wand,
        WeaponWeight::Light,
        15.0,
        22.0,
        false,
        ManaRequirement::custom(25, 25),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 3, 5),
        "Dense focus rod tuned for sustained beam work.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::LivingCodex,
        "Living Codex",
        WeaponArchetype::Caster,
        WeaponFamily::Codex,
        WeaponWeight::Light,
        13.0,
        22.0,
        false,
        ManaRequirement::custom(22, 28),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 2, 5),
        "Self-turning tome that suggests optimal spell matrices.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::StellarFocus,
        "Stellar Focus",
        WeaponArchetype::Caster,
        WeaponFamily::Focus,
        WeaponWeight::Light,
        22.0,
        23.0,
        true,
        ManaRequirement::custom(34, 18),
        WeaponDiscipline::new(ControlSpread::new(4, 3), 4, 4),
        "Floating prism aligning leyline currents mid-cast.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::FrostTome,
        "Frost Tome",
        WeaponArchetype::Caster,
        WeaponFamily::Codex,
        WeaponWeight::Light,
        16.0,
        22.0,
        false,
        ManaRequirement::custom(24, 30),
        WeaponDiscipline::new(ControlSpread::new(5, 3), 3, 4),
        "Cryomantic grimoire that rewards patience and precise glyphing.",
    ),
    // Ranger catalog
    WeaponDefinition::new(
        WeaponBlueprint::ShortBow,
        "Short Bow",
        WeaponArchetype::Ranger,
        WeaponFamily::ShortBow,
        WeaponWeight::Medium,
        24.0,
        24.0,
        false,
        ManaRequirement::custom(8, 6),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 4, 5),
        "Skirmisher bow. Great on the move with quick draws.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::LongBow,
        "Long Bow",
        WeaponArchetype::Ranger,
        WeaponFamily::LongBow,
        WeaponWeight::Medium,
        28.0,
        24.0,
        true,
        ManaRequirement::custom(12, 8),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 5, 4),
        "Siege-grade bow that rains devastation from afar.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::Crossbow,
        "Crossbow",
        WeaponArchetype::Ranger,
        WeaponFamily::Crossbow,
        WeaponWeight::Medium,
        30.0,
        24.0,
        false,
        ManaRequirement::custom(14, 8),
        WeaponDiscipline::new(ControlSpread::new(3, 2), 6, 4),
        "Crank-powered bolts for methodical, armor-piercing shots.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::CompositeBow,
        "Composite Bow",
        WeaponArchetype::Ranger,
        WeaponFamily::ShortBow,
        WeaponWeight::Medium,
        25.0,
        24.0,
        false,
        ManaRequirement::custom(9, 7),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 4, 5),
        "Layered woods and horn granting steady draw resistance.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::HawkLongbow,
        "Hawk Longbow",
        WeaponArchetype::Ranger,
        WeaponFamily::LongBow,
        WeaponWeight::Medium,
        29.0,
        24.0,
        true,
        ManaRequirement::custom(12, 9),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 5, 4),
        "Precision-crafted bow favored by royal Wardens.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::RepeatingCrossbow,
        "Repeating Crossbow",
        WeaponArchetype::Ranger,
        WeaponFamily::Crossbow,
        WeaponWeight::Medium,
        28.0,
        24.0,
        false,
        ManaRequirement::custom(14, 9),
        WeaponDiscipline::new(ControlSpread::new(3, 2), 6, 4),
        "Magazine-fed crossbow for relentless suppressive fire.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::GaleShortbow,
        "Gale Shortbow",
        WeaponArchetype::Ranger,
        WeaponFamily::ShortBow,
        WeaponWeight::Medium,
        24.0,
        24.0,
        false,
        ManaRequirement::custom(9, 6),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 3, 6),
        "Wind-tuned bow ideal for moving volleys and aerial shots.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::SiegeCrossbow,
        "Siege Crossbow",
        WeaponArchetype::Ranger,
        WeaponFamily::Crossbow,
        WeaponWeight::Heavy,
        33.0,
        25.0,
        true,
        ManaRequirement::custom(16, 10),
        WeaponDiscipline::new(ControlSpread::new(3, 2), 7, 3),
        "Shoulder-braced launcher that cracks battlements.",
    ),
    WeaponDefinition::new(
        WeaponBlueprint::SpiritLongbow,
        "Spirit Longbow",
        WeaponArchetype::Ranger,
        WeaponFamily::LongBow,
        WeaponWeight::Medium,
        27.0,
        24.0,
        false,
        ManaRequirement::custom(10, 8),
        WeaponDiscipline::new(ControlSpread::new(4, 2), 4, 5),
        "Living wood bow that hums when lined up with weak points.",
    ),
];

static WEAPON_DATABASE: WeaponDatabase = WeaponDatabase::new(WEAPON_LIBRARY);

pub fn weapon_database() -> &'static WeaponDatabase {
    &WEAPON_DATABASE
}

pub fn weapon_catalog() -> &'static [WeaponDefinition] {
    weapon_database().all()
}

pub fn weapon_variants(family: WeaponFamily) -> Vec<&'static WeaponDefinition> {
    weapon_database().by_family(family)
}

pub fn weapon_definition(name: &str) -> Option<&'static WeaponDefinition> {
    weapon_database().by_name(name)
}

pub fn weapon_definition_by_blueprint(
    blueprint: WeaponBlueprint,
) -> &'static WeaponDefinition {
    weapon_database()
        .by_blueprint(blueprint)
        .unwrap_or_else(|| default_weapon_definition())
}

fn default_weapon_definition() -> &'static WeaponDefinition {
    WEAPON_LIBRARY
        .first()
        .expect("Weapon catalog must contain at least one entry")
}

fn default_blueprint() -> WeaponBlueprint {
    default_weapon_definition().blueprint()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponInfo {
    #[serde(default = "default_blueprint")]
    blueprint: WeaponBlueprint,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default)]
    quality: WeaponQuality,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    custom_discipline: Option<WeaponDiscipline>,
    damage: f64,
    durability: f64,
    #[serde(default)]
    magic_resistance: f32,
}

impl WeaponInfo {
    pub fn new() -> Self {
        Self::from_definition(default_weapon_definition())
    }

    pub fn from_definition(definition: &WeaponDefinition) -> Self {
        Self::from_definition_with_quality(definition, WeaponQuality::default())
    }

    pub fn from_definition_with_quality(
        definition: &WeaponDefinition,
        quality: WeaponQuality,
    ) -> Self {
        let mut info = Self {
            blueprint: definition.blueprint(),
            name: None,
            quality,
            custom_discipline: None,
            damage: 0.0,
            durability: 0.0,
            magic_resistance: 0.0,
        };
        info.refresh_stats();
        info
    }

    #[allow(dead_code)]
    pub fn from_weapon_name(name: &str) -> Self {
        let definition = weapon_definition(name).unwrap_or_else(default_weapon_definition);
        Self::from_definition(definition)
    }

    #[allow(dead_code)]
    pub fn from_crude_loot(definition: &WeaponDefinition) -> Self {
        Self::from_definition_with_quality(definition, WeaponQuality::Crude)
    }

    pub fn definition(&self) -> &'static WeaponDefinition {
        weapon_definition_by_blueprint(self.blueprint)
    }

    #[allow(dead_code)]
    pub fn weight(&self) -> WeaponWeight {
        self.definition().weight
    }

    #[allow(dead_code)]
    pub fn archetype(&self) -> WeaponArchetype {
        self.definition().archetype
    }

    pub fn family(&self) -> WeaponFamily {
        self.definition().family
    }

    #[allow(dead_code)]
    pub fn blueprint(&self) -> WeaponBlueprint {
        self.blueprint
    }

    pub fn discipline(&self) -> WeaponDiscipline {
        self.custom_discipline.unwrap_or_else(|| self.definition().discipline())
    }

    #[allow(dead_code)]
    pub fn allocate_discipline(&mut self, discipline: WeaponDiscipline) {
        if discipline.total_points() != WeaponDiscipline::TOTAL_STARTER_POINTS {
            println!(
                "Weapon allocations require {} total points. Provided allocation = {}.",
                WeaponDiscipline::TOTAL_STARTER_POINTS,
                discipline.total_points()
            );
            return;
        }
        self.custom_discipline = Some(discipline);
    }

    #[allow(dead_code)]
    pub fn clear_custom_discipline(&mut self) {
        self.custom_discipline = None;
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn custom_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[allow(dead_code)]
    pub fn set_custom_name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into());
    }

    #[allow(dead_code)]
    pub fn clear_custom_name(&mut self) {
        self.name = None;
    }

    #[allow(dead_code)]
    pub fn quality(&self) -> WeaponQuality {
        self.quality
    }

    #[allow(dead_code)]
    pub fn damage(&self) -> f64 {
        self.damage
    }

    #[allow(dead_code)]
    pub fn durability(&self) -> f64 {
        self.durability
    }

    #[allow(dead_code)]
    pub fn magic_resistance(&self) -> f32 {
        self.magic_resistance
    }

    pub fn display_name(&self) -> String {
        self.custom_name()
            .map(|name| name.to_string())
            .unwrap_or_else(|| self.definition().weapon_name.to_string())
    }

    pub fn badge(&self) -> String {
        format!("{} {}", self.family().emoji(), self.display_name())
    }

    fn refresh_stats(&mut self) {
        let template = self.definition();
        let base_damage = template.base_damage;
        let base_durability = template.base_durability;
        self.damage = if self.quality == WeaponQuality::Crude {
            1.0
        } else {
            (base_damage + self.quality.damage_bonus()).max(1.0)
        };
        self.durability = (base_durability + self.quality.durability_bonus()).max(5.0);
        self.magic_resistance = self.quality.magic_resistance();
    }

    #[allow(dead_code)]
    pub fn try_reforge_crude(&mut self) -> WeaponQuality {
        if self.quality != WeaponQuality::Crude {
            println!("Only crude weapons can be reforged via salvage.");
            return self.quality;
        }
        let mut rng = rand::thread_rng();
        let roll: u16 = rng.gen_range(0..1000);
        self.quality = if roll == 0 {
            println!("Incredible craftsmanship! The crude weapon ascends to Masterwork.");
            WeaponQuality::Masterwork
        } else if roll < 10 {
            println!("Lucky strike! The weapon solidifies into Superior quality.");
            WeaponQuality::Superior
        } else if roll < 210 {
            println!("Salvage succeeded. Weapon restored to Standard quality.");
            WeaponQuality::Standard
        } else {
            println!("The reforge failed. The weapon remains crude.");
            WeaponQuality::Crude
        };
        self.refresh_stats();
        self.quality
    }
}

impl fmt::Display for WeaponInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let discipline = self.discipline();
        write!(
            f,
            "{} [{}] (Damage {:.1}, Durability {:.1}) | {}",
            self.badge(),
            self.quality,
            self.damage,
            self.durability,
            discipline.describe()
        )
    }
}

impl fmt::Display for WeaponQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WeaponQuality::Crude => "Crude",
            WeaponQuality::Standard => "Standard",
            WeaponQuality::Superior => "Superior",
            WeaponQuality::Masterwork => "Masterwork",
        };
        write!(f, "{}", label)
    }
}
