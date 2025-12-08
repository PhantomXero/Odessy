use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn void_spike(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Void Spike",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        14,
        11,
        17,
        3,
        "Condensed void bolt that unravels buffs on impact.",
    )
}
pub fn null_barrier(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Null Barrier",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        12,
        14,
        12,
        3,
        "Spell-dampening shield that weakens enemy sorcery.",
    )
}
