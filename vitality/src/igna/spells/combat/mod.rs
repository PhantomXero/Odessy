use crate::core::{VitalityElement, VitalityLevel};
use crate::spell::{build_spell, SpellRole, SpellSignature, SpellTarget};

pub fn flame_lance(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Awakened,
        "Flame Lance",
        SpellRole::Attack,
        SpellTarget::EnemyTarget,
        14,
        10,
        16,
        2,
        "Piercing stream of fire that drills through lightly armored targets.",
    )
}
pub fn cinder_guard(element: VitalityElement) -> SpellSignature {
    build_spell(
        element,
        VitalityLevel::Attuned,
        "Cinder Guard",
        SpellRole::Defense,
        SpellTarget::SelfTarget,
        12,
        14,
        12,
        3,
        "Rotating embers that shred incoming projectiles and dull melee swings.",
    )
}
