use el_roi::read_int;
use serde::{Deserialize, Serialize};
use std::fmt;

enum List {
    Nation,
    SocialClass,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Nation {
    Arigo,
}

impl fmt::Display for Nation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Nation::Arigo => "Arigo",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SocialClass {
    Royal,
    Noble,
    Military,
    Civilian,
}

impl fmt::Display for SocialClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            SocialClass::Royal => "Royal",
            SocialClass::Noble => "Noble",
            SocialClass::Military => "Military",
            SocialClass::Civilian => "Civilian",
        };
        write!(f, "{}", label)
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
        Self::lists(List::Nation);
        self.nationality = match read_int("Enter the number of your Nationality: ") {
            1 => Nation::Arigo,
            _ => Nation::Arigo,
        };
        Self::lists(List::SocialClass);
        self.social_class = match read_int("Enter the number of your Social Class: ") {
            1 => SocialClass::Civilian,
            2 => SocialClass::Military,
            3 => SocialClass::Noble,
            4 => SocialClass::Royal,
            _ => self.social_class.clone(),
        };
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
    pub fn lists(list: List) {
        match list {
            List::Nation => {
                println!("Nation");
                println!("1. {}", Nation::Arigo);
            }
            List::SocialClass => {
                println!("Social Class");
                println!("1. {}", SocialClass::Civilian);
                println!("2. {}", SocialClass::Military);
                println!("3. {}", SocialClass::Noble);
                println!("4. {}", SocialClass::Royal);
            }
        }
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
