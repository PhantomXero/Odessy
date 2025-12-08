use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn shadow_pierce(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Shadow Pierce",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        13,
        11,
        16,
        3,
        "Needle of darkness that bypasses mundane armor.",
    )
}
pub fn veil_of_night(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Veil of Night",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        12,
        15,
        12,
        3,
        "Darkening shield field that obscures allies and dampens damage.",
    )
}
