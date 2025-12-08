pub mod utility;
pub mod combat;
pub mod support;

use crate::core::VitalityElement;
use crate::spell::{ElementSpellList, ElementSpellTable, SpellRecord};

pub fn spell_table(element: VitalityElement) -> ElementSpellTable {
    ElementSpellTable::new(
        vec![
        utility::stone_touch(element),
    ],
        vec![
        combat::stone_shard(element),
    ],
        vec![
        combat::earth_shield(element),
    ],
        vec![
        support::iron_root(element),
    ],
    )
}

pub fn starter_list(element: VitalityElement) -> ElementSpellList {
    spell_table(element).into_starter_list()
}

pub fn spell_records(element: VitalityElement) -> Vec<SpellRecord> {
    spell_table(element).into_records()
}
