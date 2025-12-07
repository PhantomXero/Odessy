use el_roi::read_int;
use serde::{Deserialize, Serialize};
use std::fmt;

const VITALITY_GUIDE: &str = "Vitality defines your supernatural alignment. You can only attune to one element, and levels progress linearly through mastery.";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum VitalityLevel {
    Dormant,
    Awakened,
    Attuned,
    Channeling,
    Empowered,
    Mastered,
    Ghost,
}

impl fmt::Display for VitalityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            VitalityLevel::Dormant => "Dormant",
            VitalityLevel::Awakened => "Awakened",
            VitalityLevel::Attuned => "Attuned",
            VitalityLevel::Channeling => "Channeling",
            VitalityLevel::Empowered => "Empowered",
            VitalityLevel::Mastered => "Mastered",
            VitalityLevel::Ghost => "Ghost",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VitalityElement {
    Fire,
    Water,
    Ice,
    Earth,
    Plants,
    Air,
    Lightning,
    Void,
    Null,
}

impl fmt::Display for VitalityElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            VitalityElement::Fire => "Fire",
            VitalityElement::Water => "Water",
            VitalityElement::Ice => "Ice",
            VitalityElement::Earth => "Earth",
            VitalityElement::Plants => "Plants",
            VitalityElement::Air => "Air",
            VitalityElement::Lightning => "Lightning",
            VitalityElement::Void => "Void",
            VitalityElement::Null => "Null",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VitalityInfo {
    element: VitalityElement,
    level: VitalityLevel,
}

impl VitalityInfo {
    pub fn new(element: VitalityElement, level: VitalityLevel) -> Self {
        Self { element, level }
    }

    pub fn from_prompt() -> Self {
        println!("--- Vitality ---");
        println!("{}", VITALITY_GUIDE);
        Self::list();
        let element = match read_int("Enter the number of your Vitality Element: ") {
            1 => VitalityElement::Air,
            2 => VitalityElement::Earth,
            3 => VitalityElement::Fire,
            4 => VitalityElement::Ice,
            5 => VitalityElement::Lightning,
            6 => VitalityElement::Void,
            7 => VitalityElement::Plants,
            8 => VitalityElement::Water,
            _ => VitalityElement::Null,
        };

        Self {
            element,
            level: VitalityLevel::Dormant,
        }
    }

    pub fn element(&self) -> VitalityElement {
        self.element
    }

    pub fn level(&self) -> VitalityLevel {
        self.level
    }

    pub fn level_up(&mut self) {
        self.level = match self.level {
            VitalityLevel::Dormant => VitalityLevel::Awakened,
            VitalityLevel::Awakened => VitalityLevel::Attuned,
            VitalityLevel::Attuned => VitalityLevel::Channeling,
            VitalityLevel::Channeling => VitalityLevel::Empowered,
            VitalityLevel::Empowered => VitalityLevel::Mastered,
            VitalityLevel::Mastered => VitalityLevel::Ghost,
            VitalityLevel::Ghost => VitalityLevel::Ghost,
        };
    }

    pub fn list() {
        println!("Vitality Element");
        println!("1. {}", VitalityElement::Air);
        println!("2. {}", VitalityElement::Earth);
        println!("3. {}", VitalityElement::Fire);
        println!("4. {}", VitalityElement::Ice);
        println!("5. {}", VitalityElement::Lightning);
        println!("6. {}", VitalityElement::Void);
        println!("7. {}", VitalityElement::Plants);
        println!("8. {}", VitalityElement::Water);
    }
}

impl Default for VitalityInfo {
    fn default() -> Self {
        Self {
            element: VitalityElement::Null,
            level: VitalityLevel::Dormant,
        }
    }
}

impl fmt::Display for VitalityInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Element : {}", self.element)?;
        write!(f, "Level   : {}", self.level)
    }
}
