use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn lightning_javelin(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Lightning Javelin",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        14,
        10,
        17,
        3,
        "Focused bolt that pierces ranks and overcharges targets.",
    )
}
pub fn static_field(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Static Field",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        12,
        14,
        12,
        3,
        "Crackling aura that reduces damage and shocks attackers.",
    )
}
