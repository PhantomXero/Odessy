use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn ice_spike(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Ice Spike",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        14,
        11,
        16,
        2,
        "Shard of ice that erupts from the ground and pins enemies in place.",
    )
}
pub fn frozen_aegis(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Frozen Aegis",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        12,
        15,
        12,
        3,
        "Encases the caster in layered ice armor to blunt heavy hits.",
    )
}
