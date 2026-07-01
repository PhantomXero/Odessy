pub use ::vitality::core::{VitalityAttributes, VitalityElement, VitalityProfile};
pub use ::vitality::mana::ManaPool;
pub use ::vitality::spell::{
	basic_spell_loadout,
	SpellCombatPreference,
	SpellLoadout,
};

pub use ::vitality::VitalityInfo;
pub mod vitality;
pub use vitality::prompt_vitality_info;
