use crate::prompt::{select_from_menu, MenuItem};
use serde::{Deserialize, Serialize};
use std::fmt;

const HISTORY_GUIDE: &str = "History defines where your loyalties, taxes, and faction bonuses come from. Nations rarely change, but social class can grow with prestige.";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Nation {
    Arigo,
}

impl fmt::Display for Nation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

impl Nation {
    fn label(&self) -> &'static str {
        match self {
            Nation::Arigo => "Arigo",
        }
    }

    fn emoji(&self) -> &'static str {
        match self {
            Nation::Arigo => "🏛️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialClass {
    Royal,
    Noble,
    Military,
    Civilian,
}

impl fmt::Display for SocialClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

impl SocialClass {
    fn label(&self) -> &'static str {
        match self {
            SocialClass::Royal => "Royal",
            SocialClass::Noble => "Noble",
            SocialClass::Military => "Military",
            SocialClass::Civilian => "Civilian",
        }
    }

    fn emoji(&self) -> &'static str {
        match self {
            SocialClass::Royal => "👑",
            SocialClass::Noble => "🏰",
            SocialClass::Military => "⚔️",
            SocialClass::Civilian => "🧑",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivicInfo {
    nationality: Nation,
    social_class: SocialClass,
}

impl CivicInfo {
    pub fn new() -> Self {
        let nationality = Nation::Arigo;
        let social_class = SocialClass::Civilian;

        Self {
            nationality,
            social_class,
        }
    }
    pub fn edit(&mut self) {
        println!("--- History ---");
        println!("{}", HISTORY_GUIDE);
        let nation_choice = select_from_menu(
            "Nation",
            Some(HISTORY_GUIDE),
            &[MenuItem::with_info(
                "Arigo",
                "Heartland empire. Stable taxes, formal courts, and persistent faction wars.",
            )],
        );
        self.nationality = match nation_choice.index {
            0 => Nation::Arigo,
            _ => Nation::Arigo,
        };
        println!("Nation: {}", self.nationality);
        self.social_class = SocialClass::Civilian;
        println!(
            "Social Class: {} (all heroes begin at base rank; prestige will advance it later)",
            self.social_class
        );
    }
    pub fn level_up(&mut self) {
        match self.social_class {
            SocialClass::Civilian => self.social_class = SocialClass::Military,
            SocialClass::Military => self.social_class = SocialClass::Noble,
            SocialClass::Noble => self.social_class = SocialClass::Royal,
            _ => self.social_class = self.social_class.clone(),
        }
    }
    pub fn nationality(&self) -> Nation {
        self.nationality
    }

    pub fn social_class(&self) -> &SocialClass {
        &self.social_class
    }
}

impl Default for CivicInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CivicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nationality : {}", self.nationality)?;
        write!(f, "Social Class: {}", self.social_class)
    }
}
