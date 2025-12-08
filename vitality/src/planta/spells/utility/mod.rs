use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn verdant_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Verdant Touch",
        SpellRole::Utility,
        SpellTarget::SelfTarget,
        5,
        6,
        7,
        0,
        "Accelerates plant growth, detects roots, and softens soil.",
    )
}
