pub mod core;
pub mod mana;
pub mod spell;
pub mod aqua;
pub mod glacia;
pub mod igna;
pub mod nulla;
pub mod planta;
pub mod terra;
pub mod venta;
pub mod volt;
pub mod umbra;
pub mod profile;

pub use core::{VitalityAttributes, VitalityElement, VitalityLevel, VitalityProfile};
pub use mana::{ManaPool, ManaSpendOutcome};
pub use profile::VitalityInfo;
pub use spell::{
    all_spell_records, basic_spell_loadout, elemental_multiplier, resolve_spell_interaction,
    DurationType, InteractionResult, SpellBehavior, SpellCombatPreference, SpellInteractionOutcome,
    SpellLoadout, SpellProfile, SpellRecord, SpellRole, SpellSignature, SpellStatus,
    SpellTarget, SpellTier,
};
