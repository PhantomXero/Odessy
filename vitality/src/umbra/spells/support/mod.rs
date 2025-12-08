use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn night_shift(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Night Shift",
        SpellRole::Support,
        SpellTarget::SelfTarget,
        10,
        17,
        10,
        2,
        "Boosts evasion and reduces enemy detection range.",
    )
}
