use serde::{Deserialize, Serialize};
use std::fmt;

use crate::core::{VitalityAttributes, VitalityElement, VitalityProfile};
use crate::spell::{basic_spell_loadout, SpellCombatPreference, SpellLoadout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalityInfo {
    profile: VitalityProfile,
    #[serde(default)]
    spells: SpellLoadout,
}

impl VitalityInfo {
    pub fn new(profile: VitalityProfile, spells: SpellLoadout) -> Self {
        Self { profile, spells }
    }

    pub fn starter(
        element: VitalityElement,
        attributes: VitalityAttributes,
        preference: SpellCombatPreference,
    ) -> Self {
        let mut profile = VitalityProfile::generic();
        profile.set_element(element);
        *profile.attributes_mut() = attributes;
        let spells = basic_spell_loadout(element, preference);
        Self { profile, spells }
    }

    pub fn profile(&self) -> &VitalityProfile {
        &self.profile
    }

    pub fn profile_mut(&mut self) -> &mut VitalityProfile {
        &mut self.profile
    }

    pub fn spells(&self) -> &SpellLoadout {
        &self.spells
    }

    pub fn spells_mut(&mut self) -> &mut SpellLoadout {
        &mut self.spells
    }

    pub fn set_spells(&mut self, spells: SpellLoadout) {
        self.spells = spells;
    }

    pub fn set_profile(&mut self, profile: VitalityProfile) {
        self.profile = profile;
    }

    pub fn level_up(&mut self) {
        self.profile.advance_level();
    }
}

impl Default for VitalityInfo {
    fn default() -> Self {
        Self {
            profile: VitalityProfile::generic(),
            spells: SpellLoadout::default(),
        }
    }
}

impl fmt::Display for VitalityInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attrs = self.profile.attributes().describe();
        writeln!(f, "Element : {}", self.profile.element())?;
        writeln!(f, "Level   : {}", self.profile.level())?;
        writeln!(f, "Stats   : {}", attrs)?;
        let lines = self.spells.describe(self.profile.attributes());
        write!(f, "Spells  :\n{}", lines.join("\n"))
    }
}
