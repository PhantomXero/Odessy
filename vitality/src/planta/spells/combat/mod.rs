use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn thorn_burst(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Thorn Burst",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        12,
        11,
        14,
        2,
        "Explodes razor vines in a cone, inflicting bleed on attackers.",
    )
}
pub fn bramble_guard(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Bramble Guard",
        SpellRole::Defense,
        SpellTarget::AllyTarget,
        11,
        14,
        11,
        3,
        "Wraps the target in tough vines that slow and punish melee swings.",
    )
}
