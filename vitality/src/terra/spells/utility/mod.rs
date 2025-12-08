use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn stone_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Stone Touch",
        SpellRole::Utility,
        SpellTarget::SelfTarget,
        6,
        6,
        8,
        1,
        "Shapes pebbles, seals cracks, or sends minor tremor pings through walls.",
    )
}
