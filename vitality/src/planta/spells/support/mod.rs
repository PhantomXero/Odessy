use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn vital_bloom(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Vital Bloom",
        SpellRole::Support,
        SpellTarget::AllyTarget,
        12,
        18,
        10,
        3,
        "Slow heal-over-time aura that also boosts morale.",
    )
}
