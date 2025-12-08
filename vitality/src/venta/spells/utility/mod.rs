use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn breeze_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Breeze Touch",
        SpellRole::Utility,
        SpellTarget::SelfTarget,
        5,
        6,
        7,
        1,
        "Pushes light objects, clears smoke, and cushions jumps.",
    )
}
