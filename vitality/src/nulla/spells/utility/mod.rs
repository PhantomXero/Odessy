use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn null_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Null Touch",
        SpellRole::Utility,
        SpellTarget::EnemyTarget,
        6,
        6,
        7,
        1,
        "Disrupts magical effects, reveals enchantments, and dampens fields.",
    )
}
