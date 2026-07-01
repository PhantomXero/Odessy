use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VitalityElement {
    Igna,
    Aqua,
    Glacia,
    Terra,
    Planta,
    Venta,
    Volt,
    Umbra,
    Null,
}

impl VitalityElement {
    pub const ALL: [VitalityElement; 9] = [
        VitalityElement::Igna,
        VitalityElement::Aqua,
        VitalityElement::Glacia,
        VitalityElement::Terra,
        VitalityElement::Planta,
        VitalityElement::Venta,
        VitalityElement::Volt,
        VitalityElement::Umbra,
        VitalityElement::Null,
    ];

    pub fn all() -> &'static [VitalityElement; 9] {
        &Self::ALL
    }

    pub fn code(self) -> &'static str {
        match self {
            VitalityElement::Igna => "igna",
            VitalityElement::Aqua => "aqua",
            VitalityElement::Glacia => "glacia",
            VitalityElement::Terra => "terra",
            VitalityElement::Planta => "planta",
            VitalityElement::Venta => "venta",
            VitalityElement::Volt => "volt",
            VitalityElement::Umbra => "umbra",
            VitalityElement::Null => "null",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let lowered = code.trim().to_ascii_lowercase();
        match lowered.as_str() {
            "igna" => Some(VitalityElement::Igna),
            "aqua" => Some(VitalityElement::Aqua),
            "glacia" => Some(VitalityElement::Glacia),
            "terra" => Some(VitalityElement::Terra),
            "planta" => Some(VitalityElement::Planta),
            "venta" => Some(VitalityElement::Venta),
            "volt" => Some(VitalityElement::Volt),
            "umbra" => Some(VitalityElement::Umbra),
            "null" => Some(VitalityElement::Null),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            VitalityElement::Igna => "Igna (Fire)",
            VitalityElement::Aqua => "Aqua (Water)",
            VitalityElement::Glacia => "Glacia (Ice)",
            VitalityElement::Terra => "Terra (Earth)",
            VitalityElement::Planta => "Planta (Life)",
            VitalityElement::Venta => "Venta (Air)",
            VitalityElement::Volt => "Volt (Lightning)",
            VitalityElement::Umbra => "Umbra (Void)",
            VitalityElement::Null => "Unaligned",
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            VitalityElement::Igna => "🔥",
            VitalityElement::Aqua => "💧",
            VitalityElement::Glacia => "❄️",
            VitalityElement::Terra => "⛰️",
            VitalityElement::Planta => "🌿",
            VitalityElement::Venta => "🌪️",
            VitalityElement::Volt => "⚡",
            VitalityElement::Umbra => "🌑",
            VitalityElement::Null => "⚪",
        }
    }
}

impl fmt::Display for VitalityElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VitalityLevel {
    Dormant,
    Awakened,
    Attuned,
    Channeling,
    Empowered,
    Mastered,
    Ghost,
}

impl VitalityLevel {
    pub fn advance(self) -> Self {
        match self {
            VitalityLevel::Dormant => VitalityLevel::Awakened,
            VitalityLevel::Awakened => VitalityLevel::Attuned,
            VitalityLevel::Attuned => VitalityLevel::Channeling,
            VitalityLevel::Channeling => VitalityLevel::Empowered,
            VitalityLevel::Empowered => VitalityLevel::Mastered,
            VitalityLevel::Mastered => VitalityLevel::Ghost,
            VitalityLevel::Ghost => VitalityLevel::Ghost,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            VitalityLevel::Dormant => "dormant",
            VitalityLevel::Awakened => "awakened",
            VitalityLevel::Attuned => "attuned",
            VitalityLevel::Channeling => "channeling",
            VitalityLevel::Empowered => "empowered",
            VitalityLevel::Mastered => "mastered",
            VitalityLevel::Ghost => "ghost",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        let lowered = code.trim().to_ascii_lowercase();
        match lowered.as_str() {
            "dormant" => Some(VitalityLevel::Dormant),
            "awakened" => Some(VitalityLevel::Awakened),
            "attuned" => Some(VitalityLevel::Attuned),
            "channeling" => Some(VitalityLevel::Channeling),
            "empowered" => Some(VitalityLevel::Empowered),
            "mastered" => Some(VitalityLevel::Mastered),
            "ghost" => Some(VitalityLevel::Ghost),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            VitalityLevel::Dormant => "Dormant",
            VitalityLevel::Awakened => "Awakened",
            VitalityLevel::Attuned => "Attuned",
            VitalityLevel::Channeling => "Channeling",
            VitalityLevel::Empowered => "Empowered",
            VitalityLevel::Mastered => "Mastered",
            VitalityLevel::Ghost => "Ghost",
        }
    }
}

impl fmt::Display for VitalityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalityAttributes {
    pub control: u16,
    pub power: u16,
    #[serde(default)]
    pub range: u16,
    /// Percentage multiplier. 100 = base cooldown, lower is faster.
    pub cooldown_rate: u16,
}

impl VitalityAttributes {
    pub const STARTER_ALLOCATION: u8 = 10;

    pub fn baseline() -> Self {
        Self {
            control: 4,
            power: 4,
            range: 2,
            cooldown_rate: 100,
        }
    }

    pub fn from_points(control: u8, power: u8, range: u8) -> Self {
        Self {
            control: control as u16,
            power: power as u16,
            range: range as u16,
            cooldown_rate: 100,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "Control {} | Power {} | Range {} | Cooldown {}%",
            self.control, self.power, self.range, self.cooldown_rate
        )
    }

    pub fn scaled_cooldown(&self, base_seconds: u16) -> u16 {
        let scaled = u32::from(base_seconds) * u32::from(self.cooldown_rate);
        (scaled / 100) as u16
    }

    pub fn apply_rank_bonus(&mut self, level: VitalityLevel) {
        let bonus = match level {
            VitalityLevel::Dormant => 0,
            VitalityLevel::Awakened => 2,
            VitalityLevel::Attuned => 4,
            VitalityLevel::Channeling => 6,
            VitalityLevel::Empowered => 8,
            VitalityLevel::Mastered => 11,
            VitalityLevel::Ghost => 14,
        };
        self.control = self.control.saturating_add(bonus);
        self.power = self.power.saturating_add(bonus);
        self.range = self.range.saturating_add(bonus);
        if bonus > 0 {
            let reduction = bonus.min(25) as u16;
            self.cooldown_rate = self.cooldown_rate.saturating_sub(reduction);
        }
    }
}

impl Default for VitalityAttributes {
    fn default() -> Self {
        Self::baseline()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalityProfile {
    element: VitalityElement,
    level: VitalityLevel,
    attributes: VitalityAttributes,
}

impl VitalityProfile {
    pub fn generic() -> Self {
        Self {
            element: VitalityElement::Null,
            level: VitalityLevel::Dormant,
            attributes: VitalityAttributes::baseline(),
        }
    }

    pub fn element(&self) -> VitalityElement {
        self.element
    }

    pub fn level(&self) -> VitalityLevel {
        self.level
    }

    pub fn attributes(&self) -> &VitalityAttributes {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut VitalityAttributes {
        &mut self.attributes
    }

    pub fn set_element(&mut self, element: VitalityElement) {
        self.element = element;
    }

    pub fn advance_level(&mut self) {
        let next = self.level.advance();
        if next != self.level {
            self.level = next;
            self.attributes.apply_rank_bonus(self.level);
        }
    }
}

impl Default for VitalityProfile {
    fn default() -> Self {
        Self::generic()
    }
}
