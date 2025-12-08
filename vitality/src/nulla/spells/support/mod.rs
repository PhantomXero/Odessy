use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn quiet_zone(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Quiet Zone",
        SpellRole::Support,
        SpellTarget::AllyTarget,
        11,
        18,
        11,
        2,
        "Reduces debuff duration and muffles casting noise.",
    )
}
