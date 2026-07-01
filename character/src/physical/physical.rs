use crate::prompt::{select_from_menu, MenuItem};
use el_roi::read_float;
use serde::{Deserialize, Serialize};
use std::fmt;

const HEIGHT_MIN: f64 = 150.0;
const HEIGHT_MAX: f64 = 200.0;
const WEIGHT_MIN: f64 = 50.0;
const WEIGHT_MAX: f64 = 150.0;
const PHYSICAL_GUIDE: &str = "Body stats drive weapon access. Taller + heavier builds hit harder but lose speed. Light builds attack faster but can be overwhelmed by heavy gear.";
const SPEED_MIN: u8 = 1;
const SPEED_MAX: u8 = 100;
const STRENGTH_MIN: u8 = 1;
const STRENGTH_MAX: u8 = 100;
const MANA_PRESSURE_MIN: u8 = 1;
const MANA_PRESSURE_MAX: u8 = 100;
const STARTING_STAT_POINTS: u8 = 10;
const LEVEL_POINT_BONUS: u8 = 5;
const MAX_TOTAL_POINTS: u8 = 100;
const TEMP_BONUS_MIN: i8 = -20;
const TEMP_BONUS_MAX: i8 = 20;

enum List {
    Height,
    Weight,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DominantHand {
    Left,
    Right,
}

impl fmt::Display for DominantHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

impl DominantHand {
    fn label(self) -> &'static str {
        match self {
            DominantHand::Left => "Left",
            DominantHand::Right => "Right",
        }
    }

    fn emoji(self) -> &'static str {
        match self {
            DominantHand::Left => "👈",
            DominantHand::Right => "👉",
        }
    }

    fn other(self) -> Self {
        match self {
            DominantHand::Left => DominantHand::Right,
            DominantHand::Right => DominantHand::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkinColour {
    Black,
    White,
    Yellow,
    Red,
}

impl fmt::Display for SkinColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

impl SkinColour {
    fn label(self) -> &'static str {
        match self {
            SkinColour::Black => "Black",
            SkinColour::White => "White",
            SkinColour::Yellow => "Yellow",
            SkinColour::Red => "Red",
        }
    }

    fn emoji(self) -> &'static str {
        match self {
            SkinColour::Black => "⬛",
            SkinColour::White => "⬜",
            SkinColour::Yellow => "🟡",
            SkinColour::Red => "🔴",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Physic {
    Athletic,
    Lean,
    Muscular,
}

impl fmt::Display for Physic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Physic::Athletic => "Athletic",
            Physic::Lean => "Lean",
            Physic::Muscular => "Muscular",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
struct HandBonus {
    control: i8,
    power: i8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct HandProfile {
    control: u8,
    power: u8,
}

impl HandProfile {
    fn new(control: u8, power: u8) -> Self {
        Self { control, power }
    }

    pub fn control(&self) -> u8 {
        self.control
    }

    pub fn power(&self) -> u8 {
        self.power
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhysicalTier {
    Drifter,
    Strider,
    Vanguard,
    Titan,
    Colossus,
}

impl fmt::Display for PhysicalTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            PhysicalTier::Drifter => "Drifter",
            PhysicalTier::Strider => "Strider",
            PhysicalTier::Vanguard => "Vanguard",
            PhysicalTier::Titan => "Titan",
            PhysicalTier::Colossus => "Colossus",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PhysicalProgression {
    Training { points: u8 },
    Battle { points: u8 },
    Event { points: u8 },
    TemporaryBuff { speed: i8, strength: i8 },
    ClearTemporaryBuffs,
    HandTraining {
        hand: DominantHand,
        control: i8,
        power: i8,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PhysicalInfo {
    height: f64,
    weight: f64,
    physic: Physic,
    skin: SkinColour,
    dominant_hand: DominantHand,
    #[serde(default)]
    bonus_stat_points: u8,
    #[serde(default)]
    temporary_speed_bonus: i8,
    #[serde(default)]
    temporary_strength_bonus: i8,
    #[serde(default)]
    left_hand_bonus: HandBonus,
    #[serde(default)]
    right_hand_bonus: HandBonus,
    #[serde(default)]
    left_hand_profile: HandProfile,
    #[serde(default)]
    right_hand_profile: HandProfile,
    speed: u8,
    strength: u8,
    #[serde(default = "default_mana_thresholds")]
    mana_thresholds: ManaThresholds,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ManaThresholds {
    pub pressure: u8,
    pub recovery: u8,
}

impl Default for ManaThresholds {
    fn default() -> Self {
        Self {
            pressure: MANA_PRESSURE_MIN,
            recovery: MANA_PRESSURE_MIN,
        }
    }
}

fn default_mana_thresholds() -> ManaThresholds {
    ManaThresholds::default()
}

impl PhysicalInfo {
    pub fn new() -> Self {
        let height = 179.0;
        let weight = 65.0;
        let physic = Physic::Lean;
        let skin = SkinColour::Black;
        let dominant_hand = DominantHand::Right;

        let mut info = Self {
            height,
            weight,
            physic,
            skin,
            dominant_hand,
            bonus_stat_points: 0,
            temporary_speed_bonus: 0,
            temporary_strength_bonus: 0,
            left_hand_bonus: HandBonus::default(),
            right_hand_bonus: HandBonus::default(),
            left_hand_profile: HandProfile::default(),
            right_hand_profile: HandProfile::default(),
            speed: SPEED_MIN,
            strength: STRENGTH_MIN,
            mana_thresholds: ManaThresholds::default(),
        };
        info.recalculate_stats();
        info
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    fn total_stat_points(&self) -> u8 {
        (STARTING_STAT_POINTS + self.bonus_stat_points).min(MAX_TOTAL_POINTS)
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn skin_colour(&self) -> SkinColour {
        self.skin
    }

    pub fn physique(&self) -> Physic {
        self.physic
    }

    pub fn bmi(&self) -> f64 {
        let meters = self.height / 100.0;
        if meters == 0.0 {
            0.0
        } else {
            self.weight / (meters * meters)
        }
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn strength(&self) -> u8 {
        self.strength
    }

    pub fn mana_thresholds(&self) -> ManaThresholds {
        self.mana_thresholds
    }

    pub fn dominant_hand(&self) -> DominantHand {
        self.dominant_hand
    }

    pub fn hand_profile(&self, hand: DominantHand) -> HandProfile {
        match hand {
            DominantHand::Left => self.left_hand_profile,
            DominantHand::Right => self.right_hand_profile,
        }
    }

    pub fn is_ambidextrous(&self) -> bool {
        let left = self.left_hand_profile.power as i16 - self.right_hand_profile.power as i16;
        left.abs() <= 2
            && ((self.left_hand_profile.control as i16 - self.right_hand_profile.control as i16)
                .abs()
                <= 2)
    }

    pub fn tier(&self) -> PhysicalTier {
        tier_from_points(self.speed, self.strength)
    }

    pub fn record_training(&mut self, points: u8) -> bool {
        self.apply_progress(PhysicalProgression::Training { points })
    }

    pub fn record_battle(&mut self, points: u8) -> bool {
        self.apply_progress(PhysicalProgression::Battle { points })
    }

    pub fn apply_event_bonus(&mut self, points: u8) -> bool {
        self.apply_progress(PhysicalProgression::Event { points })
    }

    pub fn apply_temporary_buff(&mut self, speed: i8, strength: i8) -> bool {
        self.apply_progress(PhysicalProgression::TemporaryBuff { speed, strength })
    }

    pub fn clear_temporary_bonuses(&mut self) -> bool {
        self.apply_progress(PhysicalProgression::ClearTemporaryBuffs)
    }

    fn can_rank_up(&self) -> bool {
        let (speed_req, strength_req) = rank_requirement(self.physic);
        self.speed >= speed_req && self.strength >= strength_req
    }

    pub fn edit(&mut self) -> Result<(), String> {
        println!("--- Physical Profile ---");
        println!("{}", PHYSICAL_GUIDE);
        Self::list(List::Height);
        let height_res = read_float(&format!(
            "Enter your Height({HEIGHT_MIN}-{HEIGHT_MAX} cm): "
        ));
        Self::validate_height(height_res)?;
        self.height = height_res;

        Self::list(List::Weight);
        let weight_res = read_float(&format!(
            "Enter your Weight({WEIGHT_MIN}-{WEIGHT_MAX} kg): "
        ));
        Self::validate_weight(weight_res)?;
        self.weight = weight_res;

        let bmi = self.bmi();
        let recommended = recommended_physic(bmi);
        println!(
            "Suggested build: {} (BMI {:.1}). Speed and strength bonuses are based on this.",
            recommended, bmi
        );
        if prompt_yes_no("Accept suggested build?") {
            self.physic = recommended;
        } else {
            self.physic = Self::prompt_physic_choice(bmi);
        }

        let skin_entries = vec![
            (
                SkinColour::Black,
                MenuItem::with_info(
                    SkinColour::Black.to_string(),
                    "Deep melanin; resilient against arc scorch and harsh desert suns.",
                ),
            ),
            (
                SkinColour::Red,
                MenuItem::with_info(
                    SkinColour::Red.to_string(),
                    "Ruddy bronze hues common in volcanic regions. Thrives under Igna mana.",
                ),
            ),
            (
                SkinColour::White,
                MenuItem::with_info(
                    SkinColour::White.to_string(),
                    "Pale tones. Shows bruising quickly but reflects rune tattoos vividly.",
                ),
            ),
            (
                SkinColour::Yellow,
                MenuItem::with_info(
                    SkinColour::Yellow.to_string(),
                    "Golden undertones tied to eastern trade winds. Harmonizes with Venta aether.",
                ),
            ),
        ];
        let skin_menu: Vec<MenuItem> = skin_entries.iter().map(|(_, item)| item.clone()).collect();
        let skin_choice = select_from_menu("Skin Colour", None, &skin_menu);
        self.skin = skin_entries[skin_choice.index].0;
        println!("Skin Colour: {}", self.skin);

        let hand_entries = vec![
            (
                DominantHand::Left,
                MenuItem::with_info(
                    DominantHand::Left.to_string(),
                    "Southpaw stance. Control bonuses grow faster; weapon guards flip mirroring.",
                ),
            ),
            (
                DominantHand::Right,
                MenuItem::with_info(
                    DominantHand::Right.to_string(),
                    "Orthodox stance. Most training manuals and relic grips assume this hand.",
                ),
            ),
        ];
        let hand_menu: Vec<MenuItem> = hand_entries.iter().map(|(_, item)| item.clone()).collect();
        let hand_choice = select_from_menu(
            "Dominant Hand",
            Some("Ambidexterity unlocks later via focused training."),
            &hand_menu,
        );
        self.dominant_hand = hand_entries[hand_choice.index].0;
        println!(
            "Dominant Hand: {} (off-hand begins as {})",
            self.dominant_hand,
            self.dominant_hand.other()
        );

        self.recalculate_stats();
        Ok(())
    }

    pub fn level_up(&mut self) {
        if !self.can_rank_up() {
            let (speed_req, strength_req) = rank_requirement(self.physic);
            println!(
                "Physique rank up blocked. Need at least Speed {} and Strength {}.",
                speed_req, strength_req
            );
            return;
        }

        let previous_tier = self.tier();
        if self.apply_progress(PhysicalProgression::Training {
            points: LEVEL_POINT_BONUS,
        }) {
            println!(
                "Awarded {} automatic training points. Total pool: {}",
                LEVEL_POINT_BONUS,
                self.total_stat_points()
            );
        } else {
            println!(
                "Training points capped. No additional growth awarded (pool: {}).",
                self.total_stat_points()
            );
        }

        self.physic = match self.physic {
            Physic::Lean => Physic::Athletic,
            Physic::Athletic => Physic::Muscular,
            Physic::Muscular => Physic::Muscular,
        };
        self.recalculate_stats();
        let new_tier = self.tier();
        if new_tier != previous_tier {
            println!("Tier advanced from {} to {}!", previous_tier, new_tier);
        }
    }

    fn list(list: List) {
        match list {
            List::Height => {
                println!("Height Range: {HEIGHT_MIN}-{HEIGHT_MAX} cm");
            }
            List::Weight => {
                println!("Weight Range: {WEIGHT_MIN}-{WEIGHT_MAX} kg");
            }
        }
    }

    fn validate_height(value: f64) -> Result<(), String> {
        if (HEIGHT_MIN..=HEIGHT_MAX).contains(&value) {
            Ok(())
        } else {
            Err(format!(
                "Height must be within {HEIGHT_MIN}-{HEIGHT_MAX} cm"
            ))
        }
    }

    fn validate_weight(value: f64) -> Result<(), String> {
        if (WEIGHT_MIN..=WEIGHT_MAX).contains(&value) {
            Ok(())
        } else {
            Err(format!(
                "Weight must be within {WEIGHT_MIN}-{WEIGHT_MAX} kg"
            ))
        }
    }

    fn prompt_physic_choice(bmi: f64) -> Physic {
        let builds = [Physic::Athletic, Physic::Lean, Physic::Muscular];
        let items: Vec<MenuItem> = builds
            .iter()
            .map(|physic| {
                let info = match physic {
                    Physic::Athletic => "Balanced frame. Keeps speed/strength within a wide comfort zone.",
                    Physic::Lean => "Speed focused. Lower load tolerance but insane reaction time.",
                    Physic::Muscular => "Powerhouse. Trades agility for burst damage and carry weight.",
                };
                MenuItem::with_info(physic.to_string(), info)
            })
            .collect();
        loop {
            let guide = format!(
                "BMI {:.1} allows specific builds. Lean thrives under 19.5; Muscular prefers 25+.",
                bmi
            );
            let selection = select_from_menu("Select Physic", Some(&guide), &items);
            let candidate = builds[selection.index];
            if can_apply_physic(candidate, bmi) {
                println!("Physic: {}", candidate);
                return candidate;
            }
            println!(
                "Physic {} is not available for BMI {:.1}. Please adjust height/weight or pick another.",
                candidate, bmi
            );
        }
    }

    fn recalculate_stats(&mut self) {
        let bmi = self.bmi();
        let profile = stat_profile(self.physic);
        let total = self.total_stat_points().max(2);

        let mut speed_ratio = profile.speed_bias;
        let mut strength_ratio = profile.strength_bias;

        if self.height > 190.0 {
            strength_ratio += 0.05;
        } else if self.height < 165.0 {
            speed_ratio += 0.05;
        }

        if self.weight > 110.0 {
            strength_ratio += 0.05;
            speed_ratio -= 0.03;
        } else if self.weight < 65.0 {
            speed_ratio += 0.05;
            strength_ratio -= 0.03;
        }

        let ratio_sum = (speed_ratio + strength_ratio).max(0.1);
        let normalized_speed = speed_ratio / ratio_sum;
        let normalized_strength = strength_ratio / ratio_sum;

        let mut speed = (total as f32 * normalized_speed).round() as i32;
        let mut strength = (total as f32 * normalized_strength).round() as i32;
        let remainder = total as i32 - (speed + strength);
        if remainder > 0 {
            strength += remainder;
        } else if remainder < 0 {
            speed += remainder;
        }

        let (bmi_speed, bmi_strength) = bmi_adjustments(bmi);
        speed += bmi_speed;
        strength += bmi_strength;

        let (speed_cap, strength_cap) = (profile.speed_cap as i32, profile.strength_cap as i32);
        let (min_speed, min_strength) = (profile.min_speed as i32, profile.min_strength as i32);
        let (speed, strength) = rebalance_with_caps(
            speed,
            strength,
            total as i32,
            min_speed,
            min_strength,
            speed_cap,
            strength_cap,
        );

        let speed = clamp(min_speed, speed_cap, speed) + self.temporary_speed_bonus as i32;
        let strength = clamp(min_strength, strength_cap, strength)
            + self.temporary_strength_bonus as i32;

        self.speed = clamp(SPEED_MIN as i32, SPEED_MAX as i32, speed) as u8;
        self.strength = clamp(STRENGTH_MIN as i32, STRENGTH_MAX as i32, strength) as u8;
        self.rebuild_mana_thresholds();
        self.update_hand_profiles();
    }

    fn rebuild_mana_thresholds(&mut self) {
        let tier = self.tier();
        let (base_pressure, base_recovery) = match tier {
            PhysicalTier::Drifter => (8, 6),
            PhysicalTier::Strider => (14, 9),
            PhysicalTier::Vanguard => (22, 14),
            PhysicalTier::Titan => (32, 19),
            PhysicalTier::Colossus => (44, 24),
        };
        let physique_bonus = match self.physic {
            Physic::Lean => (4, 6),
            Physic::Athletic => (6, 6),
            Physic::Muscular => (8, 4),
        };
        let pressure = clamp(
            MANA_PRESSURE_MIN as i32,
            MANA_PRESSURE_MAX as i32,
            base_pressure + physique_bonus.0,
        ) as u8;
        let recovery = clamp(
            MANA_PRESSURE_MIN as i32,
            MANA_PRESSURE_MAX as i32,
            base_recovery + physique_bonus.1,
        ) as u8;
        self.mana_thresholds = ManaThresholds { pressure, recovery };
    }

    pub fn apply_progress(&mut self, progression: PhysicalProgression) -> bool {
        let mut mutated = false;
        match progression {
            PhysicalProgression::Training { points }
            | PhysicalProgression::Battle { points }
            | PhysicalProgression::Event { points } => {
                mutated = self.award_permanent_points(points);
            }
            PhysicalProgression::TemporaryBuff { speed, strength } => {
                let new_speed = clamp_i8(
                    TEMP_BONUS_MIN,
                    TEMP_BONUS_MAX,
                    self.temporary_speed_bonus.saturating_add(speed),
                );
                let new_strength = clamp_i8(
                    TEMP_BONUS_MIN,
                    TEMP_BONUS_MAX,
                    self.temporary_strength_bonus.saturating_add(strength),
                );
                if new_speed != self.temporary_speed_bonus
                    || new_strength != self.temporary_strength_bonus
                {
                    self.temporary_speed_bonus = new_speed;
                    self.temporary_strength_bonus = new_strength;
                    mutated = true;
                }
            }
            PhysicalProgression::HandTraining {
                hand,
                control,
                power,
            } => {
                let bonuses = self.hand_bonus_mut(hand);
                let new_control = clamp_i8(
                    TEMP_BONUS_MIN,
                    TEMP_BONUS_MAX,
                    bonuses.control.saturating_add(control),
                );
                let new_power = clamp_i8(
                    TEMP_BONUS_MIN,
                    TEMP_BONUS_MAX,
                    bonuses.power.saturating_add(power),
                );
                if new_control != bonuses.control || new_power != bonuses.power {
                    bonuses.control = new_control;
                    bonuses.power = new_power;
                    mutated = true;
                }
            }
            PhysicalProgression::ClearTemporaryBuffs => {
                if self.temporary_speed_bonus != 0 || self.temporary_strength_bonus != 0 {
                    self.temporary_speed_bonus = 0;
                    self.temporary_strength_bonus = 0;
                    mutated = true;
                }
            }
        }

        if mutated {
            self.recalculate_stats();
        }
        mutated
    }

    fn award_permanent_points(&mut self, points: u8) -> bool {
        if points == 0 {
            return false;
        }
        let max_bonus = MAX_TOTAL_POINTS.saturating_sub(STARTING_STAT_POINTS);
        let new_total = (self.bonus_stat_points as u16 + points as u16).min(max_bonus as u16);
        let updated = new_total as u8;
        if updated != self.bonus_stat_points {
            self.bonus_stat_points = updated;
            return true;
        }
        false
    }

    fn update_hand_profiles(&mut self) {
        let total_speed = self.speed as i32;
        let total_strength = self.strength as i32;
        let dom_control = ((total_speed as f32) * 0.6).round() as i32;
        let off_control = total_speed - dom_control;
        let dom_power = ((total_strength as f32) * 0.65).round() as i32;
        let off_power = total_strength - dom_power;

        let (left_control_base, right_control_base, left_power_base, right_power_base) =
            match self.dominant_hand {
                DominantHand::Left => (dom_control, off_control, dom_power, off_power),
                DominantHand::Right => (off_control, dom_control, off_power, dom_power),
            };

        let left_control = clamp(
            SPEED_MIN as i32,
            SPEED_MAX as i32,
            left_control_base + self.left_hand_bonus.control as i32,
        );
        let right_control = clamp(
            SPEED_MIN as i32,
            SPEED_MAX as i32,
            right_control_base + self.right_hand_bonus.control as i32,
        );

        let left_power = clamp(
            STRENGTH_MIN as i32,
            STRENGTH_MAX as i32,
            left_power_base + self.left_hand_bonus.power as i32,
        );
        let right_power = clamp(
            STRENGTH_MIN as i32,
            STRENGTH_MAX as i32,
            right_power_base + self.right_hand_bonus.power as i32,
        );

        self.left_hand_profile = HandProfile::new(left_control as u8, left_power as u8);
        self.right_hand_profile = HandProfile::new(right_control as u8, right_power as u8);
    }

    fn hand_bonus_mut(&mut self, hand: DominantHand) -> &mut HandBonus {
        match hand {
            DominantHand::Left => &mut self.left_hand_bonus,
            DominantHand::Right => &mut self.right_hand_bonus,
        }
    }
}

impl Default for PhysicalInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PhysicalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Height : {:.1} cm", self.height)?;
        writeln!(f, "Weight : {:.1} kg", self.weight)?;
        writeln!(f, "Build  : {}", self.physic)?;
        writeln!(f, "Skin   : {}", self.skin)?;
        writeln!(f, "Hand   : {}", self.dominant_hand)?;
        writeln!(f, "BMI    : {:.1}", self.bmi())?;
        writeln!(f, "Tier   : {}", self.tier())?;
        if self.temporary_speed_bonus != 0 || self.temporary_strength_bonus != 0 {
            writeln!(
                f,
                "Buffs  : Speed {:+}, Strength {:+}",
                self.temporary_speed_bonus, self.temporary_strength_bonus
            )?;
        }
        writeln!(
            f,
            "Hands  : L ctrl {} | pow {} / R ctrl {} | pow {} (Dom: {})",
            self.left_hand_profile.control,
            self.left_hand_profile.power,
            self.right_hand_profile.control,
            self.right_hand_profile.power,
            self.dominant_hand
        )?;
        writeln!(f, "Speed  : {}", self.speed)?;
        writeln!(f, "Strength: {}", self.strength)?;
        write!(
            f,
            "Mana    : Pressure {} | Recovery {}",
            self.mana_thresholds.pressure, self.mana_thresholds.recovery
        )
    }
}

struct StatProfile {
    speed_bias: f32,
    strength_bias: f32,
    min_speed: u8,
    min_strength: u8,
    speed_cap: u8,
    strength_cap: u8,
}

fn stat_profile(physic: Physic) -> StatProfile {
    match physic {
        Physic::Lean => StatProfile {
            speed_bias: 0.65,
            strength_bias: 0.35,
            min_speed: 4,
            min_strength: 2,
            speed_cap: SPEED_MAX,
            strength_cap: 55,
        },
        Physic::Athletic => StatProfile {
            speed_bias: 0.5,
            strength_bias: 0.5,
            min_speed: 3,
            min_strength: 3,
            speed_cap: 85,
            strength_cap: 85,
        },
        Physic::Muscular => StatProfile {
            speed_bias: 0.35,
            strength_bias: 0.65,
            min_speed: 2,
            min_strength: 5,
            speed_cap: 70,
            strength_cap: STRENGTH_MAX,
        },
    }
}

fn bmi_adjustments(bmi: f64) -> (i32, i32) {
    if bmi < 18.5 {
        (2, -2)
    } else if bmi < 22.0 {
        (1, -1)
    } else if bmi > 32.0 {
        (-3, 3)
    } else if bmi > 27.0 {
        (-1, 1)
    } else {
        (0, 0)
    }
}

fn rebalance_with_caps(
    mut speed: i32,
    mut strength: i32,
    total: i32,
    min_speed: i32,
    min_strength: i32,
    max_speed: i32,
    max_strength: i32,
) -> (i32, i32) {
    speed = clamp(min_speed, max_speed, speed);
    strength = clamp(min_strength, max_strength, strength);

    let sum = speed + strength;
    if sum < total {
        let mut deficit = total - sum;
        while deficit > 0 {
            if speed < max_speed {
                speed += 1;
            } else if strength < max_strength {
                strength += 1;
            } else {
                break;
            }
            deficit -= 1;
        }
    } else if sum > total {
        let mut overflow = sum - total;
        while overflow > 0 {
            if strength > min_strength && strength >= speed {
                strength -= 1;
            } else if speed > min_speed {
                speed -= 1;
            } else {
                break;
            }
            overflow -= 1;
        }
    }

    (speed, strength)
}

fn rank_requirement(physic: Physic) -> (u8, u8) {
    match physic {
        Physic::Lean => (18, 10),
        Physic::Athletic => (28, 24),
        Physic::Muscular => (36, 48),
    }
}

fn tier_from_points(speed: u8, strength: u8) -> PhysicalTier {
    let total = speed as u16 + strength as u16;
    match total {
        0..=20 => PhysicalTier::Drifter,
        21..=45 => PhysicalTier::Strider,
        46..=70 => PhysicalTier::Vanguard,
        71..=90 => PhysicalTier::Titan,
        _ => PhysicalTier::Colossus,
    }
}

fn recommended_physic(bmi: f64) -> Physic {
    if bmi < 20.0 {
        Physic::Lean
    } else if bmi < 27.5 {
        Physic::Athletic
    } else {
        Physic::Muscular
    }
}

fn can_apply_physic(physic: Physic, bmi: f64) -> bool {
    match physic {
        Physic::Lean => bmi <= 24.0,
        Physic::Athletic => bmi >= 18.0 && bmi <= 32.0,
        Physic::Muscular => bmi >= 22.0,
    }
}

fn clamp(min: i32, max: i32, value: i32) -> i32 {
    value.max(min).min(max)
}

fn clamp_i8(min: i8, max: i8, value: i8) -> i8 {
    clamp(min as i32, max as i32, value as i32) as i8
}

fn prompt_yes_no(message: &str) -> bool {
    let options = vec![
        MenuItem::with_info("Yes", "Accept the recommendation."),
        MenuItem::with_info("No", "Pick a different option manually."),
    ];
    let selection = select_from_menu(message, None, &options);
    println!("{}: {}", message, selection.label);
    selection.index == 0
}
