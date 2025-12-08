use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn shade_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Shade Touch",
        SpellRole::Utility,
        SpellTarget::SelfTarget,
        6,
        6,
        8,
        1,
        "Dims lights, cloaks lightly, and highlights weak points.",
    )
}
