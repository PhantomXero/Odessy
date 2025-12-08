use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn spark_touch(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Dormant,
        "Spark Touch",
        SpellRole::Utility,
        SpellTarget::EnemyTarget,
        6,
        6,
        8,
        1,
        "Powers devices, triggers mechanisms, or briefly stuns a touched foe.",
    )
}
