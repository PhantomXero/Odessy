use crate::stucture::vitality::VitalityLevel;

#[derive(Debug, Clone)]
enum SpellLevel {
    Basic { effect: u8 },
    Intermediate { effect: u8 },
    Advanced { effect: u8 },
    Expert { effect: u8 },
    Master { effect: u8 },
    Grandmaster,
}

#[derive(Debug, Clone)]
enum SpellType {
    Attack {
        level: SpellLevel,
        status: SpellStatus,
        modifier: i32,
        target: SpellTarget,
    },
    Defence {
        level: SpellLevel,
        status: SpellStatus,
        modifier: i32,
        target: SpellTarget,
    },
    Support {
        level: SpellLevel,
        status: SpellStatus,
        modifier: i32,
        target: SpellTarget,
    },
    Healing {
        level: SpellLevel,
        status: SpellStatus,
        modifier: i32,
        target: SpellTarget,
    },
}

#[derive(Debug, Clone)]
enum SpellStatus {
    Active,
    Inactive,
    Cooldown,
}

#[derive(Debug, Clone)]
enum SpellTarget {
    SelfTarget,
    AllyTarget,
    EnemyTarget,
}

#[derive(Debug, Clone)]
pub struct Spell {
    name: String,
    spell_type: SpellType,
    level: VitalityLevel,
}

impl Spell {
    pub fn new() -> Self {
        Self {
            name: String::from("New Spell"),
            spell_type: SpellType::Attack {
                level: SpellLevel::Basic { effect: 1 },
                status: SpellStatus::Active,
                modifier: 0,
                target: SpellTarget::EnemyTarget,
            },
            level: VitalityLevel::Dormant {
                control: 1,
                modifier: 1,
                power: 1,
            },
        }
    }
}
