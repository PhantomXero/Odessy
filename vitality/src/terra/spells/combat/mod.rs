use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn stone_shard(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Stone Shard",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        13,
        11,
        15,
        2,
        "Launches jagged earth splinters that can ricochet in tight corridors.",
    )
}
pub fn earth_shield(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Earth Shield",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        12,
        14,
        13,
        3,
        "Raises a stone slab barrier that intercepts frontal damage.",
    )
}
