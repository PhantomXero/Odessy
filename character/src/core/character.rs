use crate::nation::CivicInfo;
use crate::person::PersonalInfo;
use crate::physical::physical::PhysicalInfo;
use crate::prompt::{select_from_menu, MenuItem};
use crate::vitality::{prompt_vitality_info, ManaPool, VitalityInfo};
use crate::warrior::warrior::WarriorInfo;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::health::HealthPool;

const PROFILE_RULE: &str =
    "Level ups affect every subsystem but civilians skip combat progression.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    personal: PersonalInfo,
    vitality: VitalityInfo,
    physical: PhysicalInfo,
    civic: CivicInfo,
    warrior: Option<WarriorInfo>,
    mana: ManaPool,
    #[serde(default)]
    health: HealthPool,
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
        let mut character = Self {
            personal,
            vitality,
            physical,
            civic,
            warrior,
            mana: ManaPool::default(),
            health: HealthPool::default(),
        };
        character.sync_mana_focus();
        character.sync_health();
        character
    }

    pub fn from_prompt() -> Self {
        let personal = PersonalInfo::from_prompt();

        let mut civic = CivicInfo::new();
        civic.edit();

        let mut physical = PhysicalInfo::default();
        while let Err(err) = physical.edit() {
            println!("Physical setup error: {err}. Please try again.");
        }

        let vitality = prompt_vitality_info();

        let combatant_choice = select_from_menu(
            "Is this character a combatant?",
            Some(
                "Assigning a combat role unlocks weapons, fighting styles, and mana focus bonuses.",
            ),
            &[
                MenuItem::with_info(
                    "Yes",
                    "Create a warrior profile now to pick classes, style, and starting weapon.",
                ),
                MenuItem::with_info(
                    "No",
                    "Remain a civilian for now; you can enlist later from the character sheet.",
                ),
            ],
        );
        let wants_warrior = combatant_choice.index == 0;
        println!("Combatant?: {}", combatant_choice.label);
        let warrior = wants_warrior.then(|| WarriorInfo::from_prompt(&physical));

        let mut character = Self {
            personal,
            vitality,
            physical,
            civic,
            warrior,
            mana: ManaPool::default(),
            health: HealthPool::default(),
        };
        character.sync_mana_focus();
        character.sync_health();
        character
    }

    pub fn show_character_profile(&self) {
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

    pub fn mana(&self) -> &ManaPool {
        &self.mana
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

    pub fn edit_history(&mut self) {
        self.civic.edit();
    }

    pub fn edit_physical(&mut self) {
        while let Err(err) = self.physical.edit() {
            println!("Physical setup error: {err}. Please try again.");
        }
        if let Some(warrior) = &mut self.warrior {
            warrior.sync_with_physical(&self.physical);
        }
    }

    pub fn edit_vitality(&mut self) {
        self.vitality = prompt_vitality_info();
    }

    pub fn edit_warrior(&mut self) {
        if self.is_civilian() {
            println!("This character is currently a civilian.");
            let assign_choice = select_from_menu(
                "Assign combat role now?",
                Some("Warrior roles unlock class perks, weapons, and mana focus adjustments."),
                &[
                    MenuItem::with_info(
                        "Yes",
                        "Create a warrior profile immediately and choose combat options.",
                    ),
                    MenuItem::with_info(
                        "No",
                        "Remain a civilian until the story unlocks a combat branch.",
                    ),
                ],
            );
            println!("Assign combat role now?: {}", assign_choice.label);
            if assign_choice.index == 0 {
                self.warrior = Some(WarriorInfo::from_prompt(&self.physical));
                self.sync_mana_focus();
                self.sync_health();
            }
        } else if let Some(info) = &mut self.warrior {
            info.edit(&self.physical);
            self.sync_mana_focus();
            self.sync_health();
        }
    }

    pub fn unlock_warrior_respec(&mut self) {
        if let Some(info) = &mut self.warrior {
            info.grant_respec_token();
            println!(
                "Combat specialization reset unlocked. Visit the warrior editor to reassign classes or fighting style."
            );
        } else {
            println!("No warrior specialization assigned yet.");
        }
    }

    pub fn profile_card(&self) -> String {
        let mut sections = vec![];
        sections.push(section("Identity", &self.personal.to_string()));
        sections.push(section("Civic", &self.civic.to_string()));
        sections.push(section("Physical", &self.physical.to_string()));
        sections.push(section("Vitality", &self.vitality.to_string()));
        sections.push(section("Mana", &self.mana_summary()));
        sections.push(section("Health", &self.health_summary()));
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
        if let Some(info) = &mut self.warrior {
            info.sync_with_physical(&self.physical);
        }
        self.sync_mana_focus();
        self.sync_health();
    }

    pub fn is_civilian(&self) -> bool {
        self.warrior.is_none()
    }

    fn mana_summary(&self) -> String {
        format!(
            "Discipline: {}\nPool      : {}/{}",
            self.mana.focus_label(),
            self.mana.current(),
            self.mana.capacity()
        )
    }

    fn sync_mana_focus(&mut self) {
        let is_caster = self
            .warrior
            .as_ref()
            .map(|info| info.uses_caster_focus())
            .unwrap_or(false);
        self.mana.sync_focus(is_caster);
    }

    fn sync_health(&mut self) {
        let bonus = self
            .warrior
            .as_ref()
            .map(|info| info.hp_bonus())
            .unwrap_or(0);
        self.health.apply_bonus(bonus);
    }

    fn health_summary(&self) -> String {
        format!(
            "HP {}/{} (Cap {})",
            self.health.current(),
            self.health.max(),
            HealthPool::CAP
        )
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
            mana: ManaPool::default(),
            health: HealthPool::default(),
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
