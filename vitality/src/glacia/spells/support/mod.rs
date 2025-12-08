use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn cold_focus(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Channeling,
        "Cold Focus",
        SpellRole::Support,
        SpellTarget::SelfTarget,
        9,
        16,
        10,
        2,
        "Heightens control and steadies cooldown variance with a calm chill.",
    )
}
