use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn wind_cutter(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Wind Cutter",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        13,
        10,
        15,
        2,
        "Razor wind arc that slices at long range.",
    )
}
pub fn gale_guard(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Gale Guard",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        11,
        13,
        11,
        2,
        "Spinning air deflector that nudges arrows and spells aside.",
    )
}
