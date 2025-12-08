use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn frost_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Frost Touch",
        SpellRole::Utility,
        SpellTarget::EnemyTarget,
        6,
        7,
        7,
        1,
        "Freezes small surfaces, preserves items, or chills locks for shatter setups.",
    )
}
