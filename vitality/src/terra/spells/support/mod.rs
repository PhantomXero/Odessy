use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn iron_root(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Iron Root",
        SpellRole::Support,
        SpellTarget::SelfTarget,
        11,
        18,
        11,
        2,
        "Anchors the caster, boosting anti-stagger and physical resistance.",
    )
}
