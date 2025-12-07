use crate::physical::physical::PhysicalInfo;
use crate::vitality::vitality::VitalityInfo;
use crate::warrior::warrior::WarriorInfo;
use el_roi::read_int;
use nation::CivicInfo;
use person::PersonalInfo;
use serde::{Deserialize, Serialize};
use std::fmt;

mod nation;
mod person;
mod physical;
mod vitality;
mod warrior;

const PROFILE_RULE: &str =
    "Level ups affect every subsystem but civilians skip combat progression.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    personal: PersonalInfo,
    vitality: VitalityInfo,
    physical: PhysicalInfo,
    civic: CivicInfo,
    warrior: Option<WarriorInfo>,
}

impl Character {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_components(
        personal: PersonalInfo,
        vitality: VitalityInfo,
        physical: PhysicalInfo,
        civic: CivicInfo,
        warrior: Option<WarriorInfo>,
    ) -> Self {
        Self {
            personal,
            vitality,
            physical,
            civic,
            warrior,
        }
    }

    pub fn from_prompt() -> Self {
        let personal = PersonalInfo::from_prompt();
        let vitality = VitalityInfo::from_prompt();

        let mut physical = PhysicalInfo::new();
        if let Err(err) = physical.edit() {
            println!("Physical setup error: {}", err);
        }

        let mut civic = CivicInfo::new();
        civic.edit();

        let wants_warrior = matches!(
            read_int("Is this character a combatant? (1 Yes / 2 No): "),
            1
        );
        let warrior = wants_warrior.then(WarriorInfo::new);

        Self {
            personal,
            vitality,
            physical,
            civic,
            warrior,
        }
    }

    pub fn ShowCharacterProfile(&self) {
        println!("{}", self.profile_card());
    }
    pub fn level_up(&mut self) {
        self.vitality.level_up();
        self.physical.level_up();
        self.civic.level_up();
        if let Some(warrior) = &mut self.warrior {
            warrior.level_up();
        }
    }

    pub fn personal(&self) -> &PersonalInfo {
        &self.personal
    }

    pub fn vitality(&self) -> &VitalityInfo {
        &self.vitality
    }

    pub fn physical(&self) -> &PhysicalInfo {
        &self.physical
    }

    pub fn civic(&self) -> &CivicInfo {
        &self.civic
    }

    pub fn warrior(&self) -> Option<&WarriorInfo> {
        self.warrior.as_ref()
    }

    pub fn profile_card(&self) -> String {
        let mut sections = vec![];
        sections.push(section("Identity", &self.personal.to_string()));
        sections.push(section("Civic", &self.civic.to_string()));
        sections.push(section("Physical", &self.physical.to_string()));
        sections.push(section("Vitality", &self.vitality.to_string()));
        let combat_text = match &self.warrior {
            Some(info) => info.to_string(),
            None => "Civilian – no combat class assigned.".to_string(),
        };
        sections.push(section("Combat", &combat_text));

        let border = "=".repeat(77);
        format!(
            "{border}\nCharacter Profile\n{border}\n{}\n{border}\n{}",
            sections.join("\n\n"),
            PROFILE_RULE
        )
    }

    pub fn edit_identity(&mut self) {
        self.personal.edit_identity_section();
    }

    pub fn set_warrior(&mut self, warrior: Option<WarriorInfo>) {
        self.warrior = warrior;
    }

    pub fn is_civilian(&self) -> bool {
        self.warrior.is_none()
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            personal: PersonalInfo::default(),
            vitality: VitalityInfo::default(),
            physical: PhysicalInfo::default(),
            civic: CivicInfo::new(),
            warrior: None,
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.profile_card())
    }
}

fn section(title: &str, body: &str) -> String {
    format!("[{title}]\n{body}")
}
