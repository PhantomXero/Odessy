use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn tailwind(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Tailwind",
        SpellRole::Support,
        SpellTarget::AllyTarget,
        10,
        16,
        10,
        2,
        "Run and cast speed increase for nearby allies.",
    )
}
