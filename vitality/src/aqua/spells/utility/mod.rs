use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn flow_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Flow Touch",
        SpellRole::Utility,
        SpellTarget::AllyTarget,
        5,
        6,
        7,
        0,
        "Pushes light objects, cleans surfaces, or condenses drinkable water.",
    )
}
