use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn ignite_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Ignite Touch",
        SpellRole::Utility,
        SpellTarget::EnemyTarget,
        6,
        6,
        8,
        1,
        "Heats or ignites objects, melts thin ice, and lights torches on contact.",
    )
}
