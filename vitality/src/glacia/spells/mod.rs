pub mod utility;
pub mod combat;
pub mod support;

use crate::core::VitalityElement;
use crate::spell::{ElementSpellList, ElementSpellTable, SpellRecord};

pub fn spell_table(element: VitalityElement) -> ElementSpellTable {
    ElementSpellTable::new(
        vec![
        utility::frost_touch(element),
    ],
        vec![
        combat::ice_spike(element),
    ],
        vec![
        combat::frozen_aegis(element),
    ],
        vec![
        support::cold_focus(element),
    ],
    )
}

pub fn starter_list(element: VitalityElement) -> ElementSpellList {
    spell_table(element).into_starter_list()
}

pub fn spell_records(element: VitalityElement) -> Vec<SpellRecord> {
    spell_table(element).into_records()
}
