use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn soothing_current(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Soothing Current",
        SpellRole::Support,
        SpellTarget::AllyTarget,
        12,
        18,
        9,
        3,
        "Gentle regeneration stream that restores health and stamina over time.",
    )
}
