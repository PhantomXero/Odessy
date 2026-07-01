use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn overcharge(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Overcharge",
        SpellRole::Support,
        SpellTarget::SelfTarget,
        11,
        17,
        11,
        3,
        "Increases reaction speed and stamina recovery.",
    )
}
