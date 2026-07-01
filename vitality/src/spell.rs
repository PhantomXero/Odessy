use crate::core::{VitalityAttributes, VitalityElement, VitalityLevel};
use crate::{aqua, glacia, igna, nulla, planta, terra, venta, volt, umbra};
use rusqlite::{self, types::Type, Row};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellTier {
    Basic,
    Intermediate,
    Advanced,
    Expert,
    Master,
    Grandmaster,
}

impl SpellTier {
    pub fn label(self) -> &'static str {
        match self {
            SpellTier::Basic => "Basic",
            SpellTier::Intermediate => "Intermediate",
            SpellTier::Advanced => "Advanced",
            SpellTier::Expert => "Expert",
            SpellTier::Master => "Master",
            SpellTier::Grandmaster => "Grandmaster",
        }
    }

    pub fn default_effect(self) -> u8 {
        match self {
            SpellTier::Basic => 10,
            SpellTier::Intermediate => 14,
            SpellTier::Advanced => 18,
            SpellTier::Expert => 22,
            SpellTier::Master => 28,
            SpellTier::Grandmaster => 34,
        }
    }
}

impl fmt::Display for SpellTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellRole {
    Utility,
    Attack,
    Defense,
    Support,
    Healing,
}

impl fmt::Display for SpellRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SpellRole::Utility => "Utility",
            SpellRole::Attack => "Attack",
            SpellRole::Defense => "Defense",
            SpellRole::Support => "Support",
            SpellRole::Healing => "Healing",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellStatus {
    Active,
    Inactive,
    Cooldown,
}

impl fmt::Display for SpellStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SpellStatus::Active => "Active",
            SpellStatus::Inactive => "Inactive",
            SpellStatus::Cooldown => "Cooldown",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellTarget {
    SelfTarget,
    AllyTarget,
    EnemyTarget,
}

impl fmt::Display for SpellTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SpellTarget::SelfTarget => "Self",
            SpellTarget::AllyTarget => "Ally",
            SpellTarget::EnemyTarget => "Enemy",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DurationType {
    Instant,
    Timed(u16),
    Channel { max_seconds: u16 },
    Construct { hp: u32 },
}

impl DurationType {
    pub fn label(&self) -> &'static str {
        match self {
            DurationType::Instant => "Instant",
            DurationType::Timed(_) => "Timed",
            DurationType::Channel { .. } => "Channel",
            DurationType::Construct { .. } => "Construct",
        }
    }

    pub fn semantics(&self) -> DurationSemantics {
        match self {
            DurationType::Instant => DurationSemantics {
                persistence: EnvironmentPersistence::None,
                world_pressure: WorldPressure::Low,
                footprint: EnvironmentFootprint::None,
            },
            DurationType::Timed(_) => DurationSemantics {
                persistence: EnvironmentPersistence::Temporary,
                world_pressure: WorldPressure::Medium,
                footprint: EnvironmentFootprint::Zone,
            },
            DurationType::Channel { .. } => DurationSemantics {
                persistence: EnvironmentPersistence::Sustained,
                world_pressure: WorldPressure::High,
                footprint: EnvironmentFootprint::Zone,
            },
            DurationType::Construct { .. } => DurationSemantics {
                persistence: EnvironmentPersistence::Anchored,
                world_pressure: WorldPressure::High,
                footprint: EnvironmentFootprint::Structure,
            },
        }
    }
}

impl Default for DurationType {
    fn default() -> Self {
        DurationType::Instant
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnvironmentPersistence {
    None,
    Temporary,
    Sustained,
    Anchored,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorldPressure {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnvironmentFootprint {
    None,
    Trace,
    Zone,
    Structure,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DurationSemantics {
    pub persistence: EnvironmentPersistence,
    pub world_pressure: WorldPressure,
    pub footprint: EnvironmentFootprint,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttackBehavior {
    Pierce,
    Burst,
    Sustain,
    Jet,
    Storm,
    Crush,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DefenseBehavior {
    Wall,
    Shield,
    Absorb,
    Reflect,
    Disperse,
    Harden,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UtilityBehavior {
    Ignite,
    Shape,
    Purify,
    Chill,
    Flow,
    Grow,
    Energize,
    Obscure,
    Nullify,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SupportBehavior {
    Empower,
    Accelerate,
    Fortify,
    Regenerate,
    Stabilize,
    Surge,
    Silence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellBehavior {
    Attack(AttackBehavior),
    Defense(DefenseBehavior),
    Utility(UtilityBehavior),
    Support(SupportBehavior),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementEffect {
    Accelerate,
    Slow,
    Suspend,
    Grounding,
    Turbulent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityEffect {
    Clear,
    Obscure,
    Mirage,
    Static,
    Eclipse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureEffect {
    Scorching,
    Chilling,
    Temperate,
    Corrosive,
    Nullifying,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainTag {
    Scorch,
    Flood,
    Frost,
    Stone,
    Growth,
    Gale,
    Charge,
    Void,
}

#[derive(Debug, Clone)]
pub struct TerrainTransform {
    pub rule_id: &'static str,
    pub tag: TerrainTag,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConductivityLevel {
    Insulator,
    Low,
    Medium,
    High,
    Plasma,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialInteraction {
    Ignite,
    SteamBurst,
    Shatter,
    Quake,
    Bloom,
    Shear,
    Overload,
    Dread,
}

impl SpecialInteraction {
    pub fn modifier(self) -> f32 {
        match self {
            SpecialInteraction::Ignite => 1.1,
            SpecialInteraction::SteamBurst => 1.08,
            SpecialInteraction::Shatter => 1.12,
            SpecialInteraction::Quake => 1.06,
            SpecialInteraction::Bloom => 1.05,
            SpecialInteraction::Shear => 1.07,
            SpecialInteraction::Overload => 1.09,
            SpecialInteraction::Dread => 1.04,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementPhysicsProfile {
    pub movement_effect: MovementEffect,
    pub visibility_effect: VisibilityEffect,
    pub temperature_effect: TemperatureEffect,
    pub terrain_effects: Vec<TerrainTransform>,
    pub conductivity: ConductivityLevel,
}

#[derive(Debug, Clone)]
pub struct ElementBehaviorRule {
    pub rule_id: &'static str,
    pub behavior: SpellBehavior,
    pub modifier: f32,
    pub notes: &'static str,
}

#[derive(Debug, Clone)]
pub struct ElementInteractionRule {
    pub rule_id: &'static str,
    pub attacker: VitalityElement,
    pub defender: VitalityElement,
    pub multiplier: f32,
    pub special_effect: Option<SpecialInteraction>,
}

#[derive(Debug, Clone)]
pub struct ElementRuleSet {
    pub element: VitalityElement,
    pub physics: ElementPhysicsProfile,
    pub behaviors: Vec<ElementBehaviorRule>,
    pub interactions: Vec<ElementInteractionRule>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InteractionResult {
    FullPenetration,
    PartialPenetration,
    Blocked,
    Negated,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ElementBias {
    Strong,
    Neutral,
    Weak,
}

impl ElementBias {
    pub fn multiplier(self) -> f32 {
        match self {
            ElementBias::Strong => 1.35,
            ElementBias::Neutral => 1.0,
            ElementBias::Weak => 0.65,
        }
    }

    pub fn range(self) -> (f32, f32) {
        match self {
            ElementBias::Strong => (1.25, 1.5),
            ElementBias::Neutral => (1.0, 1.0),
            ElementBias::Weak => (0.5, 0.75),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpellProfile {
    pub tier: SpellTier,
    pub role: SpellRole,
    pub status: SpellStatus,
    pub target: SpellTarget,
    pub modifier: i16,
    pub effect: u8,
}

impl SpellProfile {
    pub fn new(tier: SpellTier, role: SpellRole) -> Self {
        let default_target = match role {
            SpellRole::Support | SpellRole::Healing => SpellTarget::AllyTarget,
            _ => SpellTarget::EnemyTarget,
        };
        Self {
            tier,
            role,
            status: SpellStatus::Active,
            target: default_target,
            modifier: 0,
            effect: tier.default_effect(),
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "{} {} [{} | Target {} | Mod {} | Effect {}]",
            self.role, self.tier, self.status, self.target, self.modifier, self.effect
        )
    }
}

impl Default for SpellProfile {
    fn default() -> Self {
        Self::new(SpellTier::Basic, SpellRole::Attack)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellSignature {
    pub name: String,
    pub element: VitalityElement,
    pub minimum_rank: VitalityLevel,
    pub mana_cost: u16,
    pub base_cooldown: u16,
    pub profile: SpellProfile,
    pub description: String,
    pub strong_against: Vec<VitalityElement>,
    pub weak_against: Vec<VitalityElement>,
    pub duration: DurationType,
    pub behaviors: Vec<SpellBehavior>,
}

impl SpellSignature {
    pub fn new(
        name: impl Into<String>,
        element: VitalityElement,
        minimum_rank: VitalityLevel,
        mana_cost: u16,
        base_cooldown: u16,
        profile: SpellProfile,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            element,
            minimum_rank,
            mana_cost,
            base_cooldown,
            profile,
            description: description.into(),
            strong_against: Vec::new(),
            weak_against: Vec::new(),
            duration: DurationType::Instant,
            behaviors: Vec::new(),
        }
    }

    pub fn describe(&self, attrs: &VitalityAttributes) -> String {
        let cooldown = attrs.scaled_cooldown(self.base_cooldown);
        format!(
            "{} [{} | Mana {} | CD {}s | {}] -> {} :: {} ({})",
            self.name,
            self.minimum_rank,
            self.mana_cost,
            cooldown,
            self.duration_summary(),
            self.profile.describe(),
            self.description,
            self.behavior_summary()
        )
    }

    pub fn duration_summary(&self) -> String {
        match self.duration {
            DurationType::Instant => "Instant".to_string(),
            DurationType::Timed(secs) => format!("Timed {}s", secs),
            DurationType::Channel { max_seconds } => format!("Channel {}s", max_seconds),
            DurationType::Construct { hp } => format!("Construct {}hp", hp),
        }
    }

    pub fn behavior_summary(&self) -> String {
        if self.behaviors.is_empty() {
            return "Unspecified".to_string();
        }
        self.behaviors
            .iter()
            .map(|behavior| match behavior {
                SpellBehavior::Attack(kind) => format!("Attack::{:?}", kind),
                SpellBehavior::Defense(kind) => format!("Defense::{:?}", kind),
                SpellBehavior::Utility(kind) => format!("Utility::{:?}", kind),
                SpellBehavior::Support(kind) => format!("Support::{:?}", kind),
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn sustain_mana_per_second(&self) -> Option<u16> {
        match self.duration {
            DurationType::Instant => None,
            DurationType::Timed(seconds) if seconds > 0 => {
                let per_second = (self.mana_cost as f32 / seconds as f32).ceil() as u16;
                Some(per_second.max(1))
            }
            DurationType::Channel { max_seconds } if max_seconds > 0 => {
                let per_second = (self.mana_cost as f32 / max_seconds as f32).ceil() as u16;
                Some(per_second.max(1))
            }
            DurationType::Construct { .. } => Some((self.mana_cost / 4).max(1)),
            _ => None,
        }
    }

    pub fn sustain_hp_penalty(&self, hp_multiplier: u8) -> Option<u16> {
        self
            .sustain_mana_per_second()
            .map(|mana| mana.saturating_mul(hp_multiplier as u16))
    }

    pub fn construct_hp(&self) -> Option<u32> {
        match self.duration {
            DurationType::Construct { hp } => Some(hp),
            _ => None,
        }
    }

    pub fn shield_decay_per_second(&self) -> f32 {
        let is_defense = self
            .behaviors
            .iter()
            .any(|behavior| matches!(behavior, SpellBehavior::Defense(_)));
        if !is_defense {
            return 0.0;
        }
        match self.duration {
            DurationType::Instant => 1.0,
            DurationType::Timed(seconds) if seconds > 0 => 1.0 / seconds as f32,
            DurationType::Channel { max_seconds } if max_seconds > 0 => 1.0 / max_seconds as f32,
            DurationType::Construct { hp } if hp > 0 => 1.0 / hp as f32,
            _ => 0.0,
        }
    }

    pub fn with_affinity(
        mut self,
        strong_against: Vec<VitalityElement>,
        weak_against: Vec<VitalityElement>,
    ) -> Self {
        self.strong_against = strong_against;
        self.weak_against = weak_against;
        self
    }

    pub fn with_duration(mut self, duration: DurationType) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_behaviors<I>(mut self, behaviors: I) -> Self
    where
        I: IntoIterator<Item = SpellBehavior>,
    {
        let mut unique = Vec::new();
        for behavior in behaviors {
            if !unique.contains(&behavior) {
                unique.push(behavior);
            }
        }
        self.behaviors = unique;
        self
    }

    pub fn add_behavior(&mut self, behavior: SpellBehavior) {
        if !self.behaviors.contains(&behavior) {
            self.behaviors.push(behavior);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellRecord {
    pub id: Option<i64>,
    pub slug: String,
    pub name: String,
    pub element: VitalityElement,
    pub minimum_rank: VitalityLevel,
    pub mana_cost: u16,
    pub base_cooldown: u16,
    pub profile: SpellProfile,
    pub description: String,
    pub strong_against: Vec<VitalityElement>,
    pub weak_against: Vec<VitalityElement>,
    pub duration: DurationType,
    pub behaviors: Vec<SpellBehavior>,
}

impl SpellRecord {
    pub fn slug_for(element: VitalityElement, name: &str) -> String {
        let sanitized = name
            .trim()
            .to_ascii_lowercase()
            .replace([' ', '/', '\\'], "_");
        format!("{}-{}", element.code(), sanitized)
    }

    pub fn with_id(mut self, id: Option<i64>) -> Self {
        self.id = id;
        self
    }

    pub fn from_signature(signature: SpellSignature) -> Self {
        let SpellSignature {
            name,
            element,
            minimum_rank,
            mana_cost,
            base_cooldown,
            profile,
            description,
            strong_against,
            weak_against,
            duration,
            behaviors,
        } = signature;

        let slug = Self::slug_for(element, &name);
        Self {
            id: None,
            slug,
            name,
            element,
            minimum_rank,
            mana_cost,
            base_cooldown,
            profile,
            description,
            strong_against,
            weak_against,
            duration,
            behaviors,
        }
    }

    pub fn into_signature(self) -> SpellSignature {
        SpellSignature::new(
            self.name,
            self.element,
            self.minimum_rank,
            self.mana_cost,
            self.base_cooldown,
            self.profile,
            self.description,
        )
        .with_affinity(self.strong_against, self.weak_against)
        .with_duration(self.duration)
        .with_behaviors(self.behaviors)
    }
}

impl TryFrom<&Row<'_>> for SpellRecord {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'_>) -> Result<Self, Self::Error> {
        let element_code: String = row.get("element")?;
        let level_code: String = row.get("minimum_rank")?;
        let profile_json: String = row.get("profile_json")?;
        let strong_json: String = row.get("strong_against")?;
        let weak_json: String = row.get("weak_against")?;
        let duration_json: String = row.get("duration")?;
        let behaviors_json: String = row.get("behaviors_json")?;

        let element = VitalityElement::from_code(&element_code).ok_or_else(|| {
            spell_conversion_error(format!("unknown element code: {element_code}"))
        })?;
        let minimum_rank = VitalityLevel::from_code(&level_code).ok_or_else(|| {
            spell_conversion_error(format!("unknown vitality level: {level_code}"))
        })?;
        let profile: SpellProfile = serde_json::from_str(&profile_json).map_err(|err| {
            spell_conversion_error(format!("invalid profile json: {err}"))
        })?;
        let strong_against: Vec<VitalityElement> =
            serde_json::from_str(&strong_json).map_err(|err| {
                spell_conversion_error(format!("invalid strong affinity json: {err}"))
            })?;
        let weak_against: Vec<VitalityElement> =
            serde_json::from_str(&weak_json).map_err(|err| {
                spell_conversion_error(format!("invalid weak affinity json: {err}"))
            })?;
        let duration: DurationType = serde_json::from_str(&duration_json).map_err(|err| {
            spell_conversion_error(format!("invalid duration json: {err}"))
        })?;
        let behaviors: Vec<SpellBehavior> =
            serde_json::from_str(&behaviors_json).map_err(|err| {
                spell_conversion_error(format!("invalid behavior json: {err}"))
            })?;

        Ok(Self {
            id: row.get("id")?,
            slug: row.get("slug")?,
            name: row.get("name")?,
            element,
            minimum_rank,
            mana_cost: row.get("mana_cost")?,
            base_cooldown: row.get("base_cooldown")?,
            profile,
            description: row.get("description")?,
            strong_against,
            weak_against,
            duration,
            behaviors,
        })
    }
}

#[derive(Debug)]
struct SpellConversionError(String);

impl fmt::Display for SpellConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SpellConversionError {}

fn spell_conversion_error(message: impl Into<String>) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(
        0,
        Type::Text,
        Box::new(SpellConversionError(message.into())),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellLoadout {
    pub utility: SpellSignature,
    pub combat: SpellSignature,
    pub support: SpellSignature,
}

impl SpellLoadout {
    pub fn new(
        utility: SpellSignature,
        combat: SpellSignature,
        support: SpellSignature,
    ) -> Self {
        Self {
            utility,
            combat,
            support,
        }
    }

    pub fn describe(&self, attrs: &VitalityAttributes) -> Vec<String> {
        vec![
            format!("Utility : {}", self.utility.describe(attrs)),
            format!("Combat  : {}", self.combat.describe(attrs)),
            format!("Support : {}", self.support.describe(attrs)),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct ElementSpellList {
    pub utility: SpellSignature,
    pub attack: SpellSignature,
    pub defense: SpellSignature,
    pub support: SpellSignature,
}

impl ElementSpellList {
    pub fn new(
        utility: SpellSignature,
        attack: SpellSignature,
        defense: SpellSignature,
        support: SpellSignature,
    ) -> Self {
        Self {
            utility,
            attack,
            defense,
            support,
        }
    }

    pub fn into_loadout(self, preference: SpellCombatPreference) -> SpellLoadout {
        let combat = match preference {
            SpellCombatPreference::Offensive => self.attack,
            SpellCombatPreference::Defensive => self.defense,
        };
        SpellLoadout::new(self.utility, combat, self.support)
    }
}

#[derive(Debug, Clone)]
pub struct ElementSpellTable {
    pub utility: Vec<SpellSignature>,
    pub attack: Vec<SpellSignature>,
    pub defense: Vec<SpellSignature>,
    pub support: Vec<SpellSignature>,
}

impl ElementSpellTable {
    pub fn new(
        utility: Vec<SpellSignature>,
        attack: Vec<SpellSignature>,
        defense: Vec<SpellSignature>,
        support: Vec<SpellSignature>,
    ) -> Self {
        Self {
            utility,
            attack,
            defense,
            support,
        }
    }

    pub fn into_starter_list(self) -> ElementSpellList {
        fn take_first(list: Vec<SpellSignature>, slot: &str) -> SpellSignature {
            list.into_iter()
                .next()
                .unwrap_or_else(|| panic!("expected at least one {} spell in table", slot))
        }

        let ElementSpellTable {
            utility,
            attack,
            defense,
            support,
        } = self;

        ElementSpellList::new(
            take_first(utility, "utility"),
            take_first(attack, "attack"),
            take_first(defense, "defense"),
            take_first(support, "support"),
        )
    }

    pub fn into_records(self) -> Vec<SpellRecord> {
        let ElementSpellTable {
            utility,
            attack,
            defense,
            support,
        } = self;

        utility
            .into_iter()
            .chain(attack)
            .chain(defense)
            .chain(support)
            .map(SpellRecord::from_signature)
            .collect()
    }
}

impl Default for SpellLoadout {
    fn default() -> Self {
        basic_spell_loadout(VitalityElement::Null, SpellCombatPreference::Defensive)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellCombatPreference {
    Offensive,
    Defensive,
}

pub fn basic_spell_loadout(
    element: VitalityElement,
    preference: SpellCombatPreference,
) -> SpellLoadout {
    elemental_spell_set(element).into_loadout(preference)
}

fn elemental_spell_set(element: VitalityElement) -> ElementSpellList {
    element_spell_table(element).into_starter_list()
}

fn element_spell_table(element: VitalityElement) -> ElementSpellTable {
    match element {
        VitalityElement::Igna => igna::spells::spell_table(element),
        VitalityElement::Aqua => aqua::spells::spell_table(element),
        VitalityElement::Glacia => glacia::spells::spell_table(element),
        VitalityElement::Terra => terra::spells::spell_table(element),
        VitalityElement::Planta => planta::spells::spell_table(element),
        VitalityElement::Venta => venta::spells::spell_table(element),
        VitalityElement::Volt => volt::spells::spell_table(element),
        VitalityElement::Umbra => umbra::spells::spell_table(element),
        VitalityElement::Null => nulla::spells::spell_table(element),
    }
}

pub fn all_spell_records() -> Vec<SpellRecord> {
    VitalityElement::ALL
        .iter()
        .flat_map(|&element| element_spell_table(element).into_records())
        .collect()
}

static ELEMENT_RULEBOOK: OnceLock<HashMap<VitalityElement, ElementRuleSet>> = OnceLock::new();

fn element_rulebook() -> &'static HashMap<VitalityElement, ElementRuleSet> {
    ELEMENT_RULEBOOK.get_or_init(build_element_rulebook)
}

fn build_element_rulebook() -> HashMap<VitalityElement, ElementRuleSet> {
    let physics = physics_seed_map();
    let behaviors = behavior_seed_map();
    let interactions = interaction_seed_data();

    let mut map = HashMap::new();
    for element in VitalityElement::ALL.iter().copied() {
        let physics_profile = physics
            .get(&element)
            .cloned()
            .unwrap_or_else(|| ElementPhysicsProfile {
                movement_effect: MovementEffect::Grounding,
                visibility_effect: VisibilityEffect::Clear,
                temperature_effect: TemperatureEffect::Temperate,
                terrain_effects: Vec::new(),
                conductivity: ConductivityLevel::Medium,
            });
        let behavior_rules = behaviors.get(&element).cloned().unwrap_or_default();
        let interaction_rules = interactions
            .iter()
            .filter(|rule| rule.attacker == element)
            .cloned()
            .collect();
        map.insert(
            element,
            ElementRuleSet {
                element,
                physics: physics_profile,
                behaviors: behavior_rules,
                interactions: interaction_rules,
            },
        );
    }
    map
}

fn physics_seed_map() -> HashMap<VitalityElement, ElementPhysicsProfile> {
    use VitalityElement::*;
    vec![
        (
            Igna,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Accelerate,
                visibility_effect: VisibilityEffect::Mirage,
                temperature_effect: TemperatureEffect::Scorching,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "IGNA-01",
                    tag: TerrainTag::Scorch,
                    description: "Cinders linger and overheat the ground",
                }],
                conductivity: ConductivityLevel::Plasma,
            },
        ),
        (
            Aqua,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Slow,
                visibility_effect: VisibilityEffect::Obscure,
                temperature_effect: TemperatureEffect::Temperate,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "AQUA-01",
                    tag: TerrainTag::Flood,
                    description: "Flooding currents reshape the arena",
                }],
                conductivity: ConductivityLevel::High,
            },
        ),
        (
            Glacia,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Grounding,
                visibility_effect: VisibilityEffect::Static,
                temperature_effect: TemperatureEffect::Chilling,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "GLAC-01",
                    tag: TerrainTag::Frost,
                    description: "Frost plates form slick cover",
                }],
                conductivity: ConductivityLevel::Low,
            },
        ),
        (
            Terra,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Grounding,
                visibility_effect: VisibilityEffect::Clear,
                temperature_effect: TemperatureEffect::Temperate,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "TERRA-01",
                    tag: TerrainTag::Stone,
                    description: "Stone pillars rise from the battlefield",
                }],
                conductivity: ConductivityLevel::Insulator,
            },
        ),
        (
            Planta,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Slow,
                visibility_effect: VisibilityEffect::Obscure,
                temperature_effect: TemperatureEffect::Temperate,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "PLANTA-01",
                    tag: TerrainTag::Growth,
                    description: "Overgrowth creates living cover",
                }],
                conductivity: ConductivityLevel::Medium,
            },
        ),
        (
            Venta,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Turbulent,
                visibility_effect: VisibilityEffect::Mirage,
                temperature_effect: TemperatureEffect::Temperate,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "VENTA-01",
                    tag: TerrainTag::Gale,
                    description: "Shearing winds erode footing",
                }],
                conductivity: ConductivityLevel::Low,
            },
        ),
        (
            Volt,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Accelerate,
                visibility_effect: VisibilityEffect::Static,
                temperature_effect: TemperatureEffect::Corrosive,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "VOLT-01",
                    tag: TerrainTag::Charge,
                    description: "Charged lattice amplifies shocks",
                }],
                conductivity: ConductivityLevel::Plasma,
            },
        ),
        (
            Umbra,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Suspend,
                visibility_effect: VisibilityEffect::Eclipse,
                temperature_effect: TemperatureEffect::Nullifying,
                terrain_effects: vec![TerrainTransform {
                    rule_id: "UMBRA-01",
                    tag: TerrainTag::Void,
                    description: "Shadow wells dampen color and sound",
                }],
                conductivity: ConductivityLevel::Low,
            },
        ),
        (
            Null,
            ElementPhysicsProfile {
                movement_effect: MovementEffect::Grounding,
                visibility_effect: VisibilityEffect::Clear,
                temperature_effect: TemperatureEffect::Temperate,
                terrain_effects: Vec::new(),
                conductivity: ConductivityLevel::Medium,
            },
        ),
    ]
    .into_iter()
    .collect()
}

fn behavior_seed_map() -> HashMap<VitalityElement, Vec<ElementBehaviorRule>> {
    use VitalityElement::*;
    vec![
        (
            Igna,
            vec![
                ElementBehaviorRule {
                    rule_id: "IGNA-02",
                    behavior: SpellBehavior::Attack(AttackBehavior::Burst),
                    modifier: 1.12,
                    notes: "Burst ignites residual fuel",
                },
                ElementBehaviorRule {
                    rule_id: "IGNA-03",
                    behavior: SpellBehavior::Defense(DefenseBehavior::Shield),
                    modifier: 0.95,
                    notes: "Heat shimmer shields decay faster",
                },
            ],
        ),
        (
            Aqua,
            vec![
                ElementBehaviorRule {
                    rule_id: "AQUA-02",
                    behavior: SpellBehavior::Utility(UtilityBehavior::Flow),
                    modifier: 1.1,
                    notes: "Flow utilities expand coverage",
                },
                ElementBehaviorRule {
                    rule_id: "AQUA-03",
                    behavior: SpellBehavior::Attack(AttackBehavior::Jet),
                    modifier: 1.05,
                    notes: "Jet attacks gain laminar precision",
                },
            ],
        ),
        (
            Glacia,
            vec![
                ElementBehaviorRule {
                    rule_id: "GLAC-02",
                    behavior: SpellBehavior::Attack(AttackBehavior::Sustain),
                    modifier: 1.08,
                    notes: "Sustained ice layers reinforce",
                },
                ElementBehaviorRule {
                    rule_id: "GLAC-03",
                    behavior: SpellBehavior::Defense(DefenseBehavior::Wall),
                    modifier: 1.1,
                    notes: "Glacial walls resist erosion",
                },
            ],
        ),
        (
            Terra,
            vec![
                ElementBehaviorRule {
                    rule_id: "TERRA-02",
                    behavior: SpellBehavior::Defense(DefenseBehavior::Harden),
                    modifier: 1.15,
                    notes: "Earthen armor densifies under pressure",
                },
            ],
        ),
        (
            Planta,
            vec![
                ElementBehaviorRule {
                    rule_id: "PLANTA-02",
                    behavior: SpellBehavior::Support(SupportBehavior::Regenerate),
                    modifier: 1.12,
                    notes: "Regrowth accelerates recovery",
                },
            ],
        ),
        (
            Venta,
            vec![
                ElementBehaviorRule {
                    rule_id: "VENTA-02",
                    behavior: SpellBehavior::Attack(AttackBehavior::Jet),
                    modifier: 1.07,
                    notes: "Jet strikes shear armor",
                },
            ],
        ),
        (
            Volt,
            vec![
                ElementBehaviorRule {
                    rule_id: "VOLT-02",
                    behavior: SpellBehavior::Attack(AttackBehavior::Storm),
                    modifier: 1.1,
                    notes: "Storm behavior builds charge",
                },
            ],
        ),
        (
            Umbra,
            vec![
                ElementBehaviorRule {
                    rule_id: "UMBRA-02",
                    behavior: SpellBehavior::Utility(UtilityBehavior::Obscure),
                    modifier: 1.15,
                    notes: "Obscurity folds perception",
                },
            ],
        ),
    ]
    .into_iter()
    .collect()
}

fn interaction_seed_data() -> Vec<ElementInteractionRule> {
    use SpecialInteraction::*;
    use VitalityElement::*;
    vec![
        ElementInteractionRule {
            rule_id: "IGNA-STR-PLANTA",
            attacker: Igna,
            defender: Planta,
            multiplier: 1.35,
            special_effect: Some(Ignite),
        },
        ElementInteractionRule {
            rule_id: "IGNA-STR-GLACIA",
            attacker: Igna,
            defender: Glacia,
            multiplier: 1.25,
            special_effect: Some(Shatter),
        },
        ElementInteractionRule {
            rule_id: "IGNA-WK-AQUA",
            attacker: Igna,
            defender: Aqua,
            multiplier: 0.75,
            special_effect: Some(SteamBurst),
        },
        ElementInteractionRule {
            rule_id: "AQUA-STR-IGNA",
            attacker: Aqua,
            defender: Igna,
            multiplier: 1.3,
            special_effect: Some(SteamBurst),
        },
        ElementInteractionRule {
            rule_id: "AQUA-STR-TERRA",
            attacker: Aqua,
            defender: Terra,
            multiplier: 1.2,
            special_effect: Some(Quake),
        },
        ElementInteractionRule {
            rule_id: "AQUA-WK-VOLT",
            attacker: Aqua,
            defender: Volt,
            multiplier: 0.7,
            special_effect: Some(Overload),
        },
        ElementInteractionRule {
            rule_id: "GLAC-STR-VENTA",
            attacker: Glacia,
            defender: Venta,
            multiplier: 1.25,
            special_effect: Some(Shatter),
        },
        ElementInteractionRule {
            rule_id: "GLAC-WK-IGNA",
            attacker: Glacia,
            defender: Igna,
            multiplier: 0.7,
            special_effect: Some(Ignite),
        },
        ElementInteractionRule {
            rule_id: "TERRA-STR-VOLT",
            attacker: Terra,
            defender: Volt,
            multiplier: 1.2,
            special_effect: Some(Quake),
        },
        ElementInteractionRule {
            rule_id: "TERRA-WK-AQUA",
            attacker: Terra,
            defender: Aqua,
            multiplier: 0.75,
            special_effect: Some(SteamBurst),
        },
        ElementInteractionRule {
            rule_id: "PLANTA-STR-TERRA",
            attacker: Planta,
            defender: Terra,
            multiplier: 1.2,
            special_effect: Some(Bloom),
        },
        ElementInteractionRule {
            rule_id: "PLANTA-WK-IGNA",
            attacker: Planta,
            defender: Igna,
            multiplier: 0.65,
            special_effect: Some(Ignite),
        },
        ElementInteractionRule {
            rule_id: "VENTA-STR-IGNA",
            attacker: Venta,
            defender: Igna,
            multiplier: 1.15,
            special_effect: Some(Shear),
        },
        ElementInteractionRule {
            rule_id: "VENTA-WK-GLACIA",
            attacker: Venta,
            defender: Glacia,
            multiplier: 0.8,
            special_effect: Some(Shatter),
        },
        ElementInteractionRule {
            rule_id: "VOLT-STR-AQUA",
            attacker: Volt,
            defender: Aqua,
            multiplier: 1.3,
            special_effect: Some(Overload),
        },
        ElementInteractionRule {
            rule_id: "VOLT-WK-TERRA",
            attacker: Volt,
            defender: Terra,
            multiplier: 0.78,
            special_effect: Some(Quake),
        },
        ElementInteractionRule {
            rule_id: "UMBRA-STR-AQUA",
            attacker: Umbra,
            defender: Aqua,
            multiplier: 1.18,
            special_effect: Some(Dread),
        },
        ElementInteractionRule {
            rule_id: "UMBRA-WK-PLANTA",
            attacker: Umbra,
            defender: Planta,
            multiplier: 0.82,
            special_effect: Some(Bloom),
        },
    ]
}

pub fn element_rule_set(element: VitalityElement) -> Option<&'static ElementRuleSet> {
    element_rulebook().get(&element)
}

fn find_interaction_rule(
    attacker: VitalityElement,
    defender: VitalityElement,
) -> Option<&'static ElementInteractionRule> {
    element_rule_set(attacker)?
        .interactions
        .iter()
        .find(|rule| rule.defender == defender)
}

fn interaction_multiplier_with_rule(
    attack: &VitalityElement,
    defense: &VitalityElement,
    spell: &SpellSignature,
) -> (f32, Option<&'static ElementInteractionRule>) {
    if *attack == VitalityElement::Null || *defense == VitalityElement::Null {
        return (1.0, None);
    }
    if let Some(rule) = find_interaction_rule(*attack, *defense) {
        return (rule.multiplier, Some(rule));
    }
    (elemental_bias(attack, defense, spell).multiplier(), None)
}

pub(crate) fn build_spell(
    element: VitalityElement,
    minimum_rank: VitalityLevel,
    name: &str,
    role: SpellRole,
    target: SpellTarget,
    mana_cost: u16,
    cooldown: u16,
    effect: u8,
    modifier: i16,
    description: &str,
) -> SpellSignature {
    let mut profile = SpellProfile::new(SpellTier::Basic, role);
    profile.target = target;
    profile.effect = effect;
    profile.modifier = modifier;
    let (strong, weak) = element_affinity(element);
    SpellSignature::new(
        name,
        element,
        minimum_rank,
        mana_cost,
        cooldown,
        profile,
        description,
    )
    .with_affinity(strong.to_vec(), weak.to_vec())
    .with_duration(default_duration_for_role(role))
    .with_behaviors(default_behaviors_for_role(role))
}

fn default_duration_for_role(role: SpellRole) -> DurationType {
    match role {
        SpellRole::Defense => DurationType::Timed(12),
        SpellRole::Support => DurationType::Channel { max_seconds: 8 },
        SpellRole::Healing => DurationType::Channel { max_seconds: 6 },
        _ => DurationType::Instant,
    }
}

fn default_behaviors_for_role(role: SpellRole) -> Vec<SpellBehavior> {
    match role {
        SpellRole::Attack => vec![SpellBehavior::Attack(AttackBehavior::Burst)],
        SpellRole::Defense => vec![SpellBehavior::Defense(DefenseBehavior::Shield)],
        SpellRole::Utility => vec![SpellBehavior::Utility(UtilityBehavior::Shape)],
        SpellRole::Support => vec![SpellBehavior::Support(SupportBehavior::Fortify)],
        SpellRole::Healing => vec![SpellBehavior::Support(SupportBehavior::Regenerate)],
    }
}

fn element_affinity(
    element: VitalityElement,
) -> (&'static [VitalityElement], &'static [VitalityElement]) {
    match element {
        VitalityElement::Igna => (
            &[VitalityElement::Planta, VitalityElement::Glacia],
            &[VitalityElement::Aqua, VitalityElement::Terra],
        ),
        VitalityElement::Aqua => (
            &[VitalityElement::Igna, VitalityElement::Terra],
            &[VitalityElement::Volt, VitalityElement::Umbra],
        ),
        VitalityElement::Glacia => (
            &[VitalityElement::Venta, VitalityElement::Volt],
            &[VitalityElement::Igna, VitalityElement::Planta],
        ),
        VitalityElement::Terra => (
            &[VitalityElement::Volt, VitalityElement::Umbra],
            &[VitalityElement::Aqua, VitalityElement::Planta],
        ),
        VitalityElement::Planta => (
            &[VitalityElement::Terra, VitalityElement::Null],
            &[VitalityElement::Igna, VitalityElement::Glacia],
        ),
        VitalityElement::Venta => (
            &[VitalityElement::Igna, VitalityElement::Aqua],
            &[VitalityElement::Glacia, VitalityElement::Volt],
        ),
        VitalityElement::Volt => (
            &[VitalityElement::Aqua, VitalityElement::Venta],
            &[VitalityElement::Terra, VitalityElement::Null],
        ),
        VitalityElement::Umbra => (
            &[VitalityElement::Aqua, VitalityElement::Volt],
            &[VitalityElement::Igna, VitalityElement::Planta],
        ),
        VitalityElement::Null => (&[], &[]),
    }
}

pub fn elemental_multiplier(
    attack: &VitalityElement,
    defense: &VitalityElement,
    spell: &SpellSignature,
) -> f32 {
    interaction_multiplier_with_rule(attack, defense, spell).0
}

fn elemental_bias(
    attack: &VitalityElement,
    defense: &VitalityElement,
    spell: &SpellSignature,
) -> ElementBias {
    if spell.strong_against.contains(defense) {
        ElementBias::Strong
    } else if spell.weak_against.contains(defense) {
        ElementBias::Weak
    } else if attack == defense {
        ElementBias::Neutral
    } else {
        ElementBias::Neutral
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpellInteractionOutcome {
    pub attack_strength: f32,
    pub defense_strength: f32,
    pub multiplier: f32,
    pub behavior_multiplier: f32,
    pub result: InteractionResult,
}

pub fn resolve_spell_interaction(
    attacking_spell: &SpellSignature,
    defending_spell: Option<&SpellSignature>,
    defender_element: VitalityElement,
    weapon_magic_resistance: f32,
) -> SpellInteractionOutcome {
    let defense_behaviors = defending_spell
        .map(|spell| spell.behaviors.clone())
        .unwrap_or_default();
    let (multiplier, interaction_rule) =
        interaction_multiplier_with_rule(&attacking_spell.element, &defender_element, attacking_spell);
    let special_modifier = interaction_rule
        .and_then(|rule| rule.special_effect)
        .map(|effect| effect.modifier())
        .unwrap_or(1.0);
    let behavior_multiplier = behavior_bias(&attacking_spell.behaviors, &defense_behaviors) * special_modifier;
    let duration_modifier = duration_conflict_modifier(
        attacking_spell,
        defending_spell,
        attacking_spell.element,
        defender_element,
    );
    let resistance_factor = (1.0 - weapon_magic_resistance).clamp(0.2, 1.0);

    let attack_strength = effective_strength(&attacking_spell.profile)
        * multiplier
        * behavior_multiplier
        * duration_modifier
        * resistance_factor;

    let defense_strength = defending_spell
        .map(|spell| {
            let behavior_guard = defense_bias(&spell.behaviors);
            effective_strength(&spell.profile) * behavior_guard
        })
        .unwrap_or(0.0);

    let result = classify_interaction(attack_strength, defense_strength);

    SpellInteractionOutcome {
        attack_strength,
        defense_strength,
        multiplier,
        behavior_multiplier,
        result,
    }
}

fn effective_strength(profile: &SpellProfile) -> f32 {
    let base = profile.effect as f32 + profile.modifier as f32;
    base.max(1.0)
}

fn classify_interaction(attack_strength: f32, defense_strength: f32) -> InteractionResult {
    if defense_strength <= 0.0 {
        return InteractionResult::FullPenetration;
    }
    let ratio = attack_strength / defense_strength;
    if ratio >= 1.25 {
        InteractionResult::FullPenetration
    } else if ratio >= 0.8 {
        InteractionResult::PartialPenetration
    } else if ratio >= 0.5 {
        InteractionResult::Blocked
    } else {
        InteractionResult::Negated
    }
}

fn behavior_bias(
    attacking: &[SpellBehavior],
    defending: &[SpellBehavior],
) -> f32 {
    if attacking.is_empty() {
        return 1.0;
    }
    let mut modifier = 1.0;
    for behavior in attacking {
        modifier *= match behavior {
            SpellBehavior::Attack(AttackBehavior::Pierce) => {
                if defending.iter().any(|b| matches!(b, SpellBehavior::Defense(DefenseBehavior::Wall | DefenseBehavior::Harden))) {
                    1.2
                } else {
                    1.05
                }
            }
            SpellBehavior::Attack(AttackBehavior::Burst) => {
                if defending
                    .iter()
                    .any(|b| matches!(b, SpellBehavior::Defense(DefenseBehavior::Disperse)))
                {
                    0.9
                } else {
                    1.0
                }
            }
            SpellBehavior::Attack(AttackBehavior::Sustain) => 1.1,
            SpellBehavior::Attack(AttackBehavior::Jet) => 1.05,
            SpellBehavior::Attack(AttackBehavior::Storm) => 1.15,
            SpellBehavior::Attack(AttackBehavior::Crush) => 1.1,
            SpellBehavior::Defense(DefenseBehavior::Reflect) => 0.95,
            SpellBehavior::Defense(_) => 1.0,
            SpellBehavior::Support(SupportBehavior::Empower) => 1.05,
            SpellBehavior::Support(_) => 1.0,
            SpellBehavior::Utility(UtilityBehavior::Nullify) => 1.05,
            SpellBehavior::Utility(UtilityBehavior::Obscure) => 1.02,
            SpellBehavior::Utility(_) => 1.0,
        };
    }
    modifier * defense_penalty(defending)
}

fn defense_penalty(defending: &[SpellBehavior]) -> f32 {
    if defending.is_empty() {
        return 1.0;
    }
    let mut modifier: f32 = 1.0;
    for behavior in defending {
        modifier *= match behavior {
            SpellBehavior::Defense(DefenseBehavior::Reflect) => 1.05,
            SpellBehavior::Defense(DefenseBehavior::Absorb) => 1.1,
            SpellBehavior::Defense(DefenseBehavior::Shield) => 1.0,
            SpellBehavior::Defense(DefenseBehavior::Disperse) => 0.95,
            SpellBehavior::Defense(DefenseBehavior::Wall) => 1.05,
            SpellBehavior::Defense(DefenseBehavior::Harden) => 1.15,
            SpellBehavior::Attack(_) => 0.95,
            SpellBehavior::Utility(UtilityBehavior::Nullify) => 1.08,
            SpellBehavior::Utility(_) => 1.0,
            SpellBehavior::Support(SupportBehavior::Stabilize) => 1.05,
            SpellBehavior::Support(_) => 1.0,
        };
    }
    modifier.clamp(0.6, 1.5)
}

fn defense_bias(defending: &[SpellBehavior]) -> f32 {
    if defending.is_empty() {
        return 1.0;
    }
    defense_penalty(defending)
}

fn duration_conflict_modifier(
    attacking_spell: &SpellSignature,
    defending_spell: Option<&SpellSignature>,
    attacker_element: VitalityElement,
    defender_element: VitalityElement,
) -> f32 {
    if attacker_element == VitalityElement::Null || defender_element == VitalityElement::Null {
        return 1.0;
    }
    let attack_semantics = attacking_spell.duration.semantics();
    let defense_semantics = defending_spell.map(|spell| spell.duration.semantics());

    let persistence_modifier = defense_semantics
        .map(|defense|
            match attack_semantics.persistence.cmp(&defense.persistence) {
                Ordering::Greater => 1.06,
                Ordering::Less => 0.95,
                Ordering::Equal => 1.0,
            }
        )
        .unwrap_or(1.03);

    let pressure_modifier = defense_semantics
        .map(|defense| match (attack_semantics.world_pressure, defense.world_pressure) {
            (WorldPressure::High, WorldPressure::Low) => 1.08,
            (WorldPressure::Medium, WorldPressure::Low) => 1.04,
            (WorldPressure::Low, WorldPressure::High) => 0.9,
            (WorldPressure::Low, WorldPressure::Medium) => 0.95,
            (WorldPressure::High, WorldPressure::High) => 1.0,
            (WorldPressure::Medium, WorldPressure::Medium) => 1.0,
            _ => 1.0,
        })
        .unwrap_or_else(|| match attack_semantics.world_pressure {
            WorldPressure::High => 1.07,
            WorldPressure::Medium => 1.03,
            WorldPressure::Low => 1.0,
        });

    let terrain_modifier = terrain_conflict_modifier(attacker_element, defender_element);
    (persistence_modifier * pressure_modifier * terrain_modifier).clamp(0.8, 1.2)
}

fn terrain_conflict_modifier(
    attacker_element: VitalityElement,
    defender_element: VitalityElement,
) -> f32 {
    let attacker_rules = match element_rule_set(attacker_element) {
        Some(rules) => rules,
        None => return 1.0,
    };
    let defender_rules = match element_rule_set(defender_element) {
        Some(rules) => rules,
        None => return 1.0,
    };
    if attacker_rules.physics.terrain_effects.is_empty() {
        return 1.0;
    }
    let overlap = attacker_rules.physics.terrain_effects.iter().any(|attack_tag| {
        defender_rules
            .physics
            .terrain_effects
            .iter()
            .any(|defend_tag| defend_tag.tag == attack_tag.tag)
    });
    let terrain_modifier = if overlap { 1.0 } else { 1.03 };
    let conductivity_modifier = match attacker_rules
        .physics
        .conductivity
        .cmp(&defender_rules.physics.conductivity)
    {
        Ordering::Greater => 1.04,
        Ordering::Less => 0.96,
        Ordering::Equal => 1.0,
    };
    terrain_modifier * conductivity_modifier
}
