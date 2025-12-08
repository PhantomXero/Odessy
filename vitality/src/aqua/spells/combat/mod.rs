use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn water_arrow(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Water Arrow",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        13,
        10,
        15,
        1,
        "High-speed water jet that staggers and drenches foes for combo setups.",
    )
}
pub fn aqua_veil(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Aqua Veil",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        11,
        13,
        11,
        2,
        "Flowing water shield that soaks arrows and redistributes impact force.",
    )
}
