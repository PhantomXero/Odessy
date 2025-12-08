use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn blaze_step(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Blaze Step",
        SpellRole::Support,
        SpellTarget::SelfTarget,
        10,
        18,
        10,
        2,
        "Surges movement speed and grants a brief fire resistance halo.",
    )
}
