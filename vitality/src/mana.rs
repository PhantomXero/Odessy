use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ManaPool {
    is_caster: bool,
    base_capacity: u16,
    bonus_capacity: i16,
    current: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ManaSpendOutcome {
    ManaOnly { mana_spent: u16 },
    OverflowToHealth { mana_spent: u16, hp_cost: u16 },
    Insufficient { missing: u16 },
}

impl ManaPool {
    const CASTER_START: u16 = 10;
    const NON_CASTER_START: u16 = 6;
    const CASTER_CAP: u16 = 110;
    const NON_CASTER_CAP: u16 = 100;
    pub const DEFAULT_HP_MULTIPLIER: u8 = 3;

    pub fn new(is_caster: bool) -> Self {
        let base_capacity = Self::starting_capacity(is_caster);
        Self {
            is_caster,
            base_capacity,
            bonus_capacity: 0,
            current: base_capacity,
        }
    }

    pub fn is_caster(&self) -> bool {
        self.is_caster
    }

    pub fn focus_label(&self) -> &'static str {
        if self.is_caster {
            "Caster"
        } else {
            "Non-Caster"
        }
    }

    fn starting_capacity(is_caster: bool) -> u16 {
        if is_caster {
            Self::CASTER_START
        } else {
            Self::NON_CASTER_START
        }
    }

    fn cap_value(is_caster: bool) -> u16 {
        if is_caster {
            Self::CASTER_CAP
        } else {
            Self::NON_CASTER_CAP
        }
    }

    pub fn capacity(&self) -> u16 {
        let total = self.base_capacity as i32 + self.bonus_capacity as i32;
        let cap = Self::cap_value(self.is_caster) as i32;
        total.clamp(0, cap) as u16
    }

    pub fn current(&self) -> u16 {
        self.current.min(self.capacity())
    }

    pub fn spend(&mut self, amount: u16) -> bool {
        if amount > self.current() {
            return false;
        }
        self.current = self.current().saturating_sub(amount);
        true
    }

    pub fn spend_for_spell(&mut self, mana_cost: u16) -> ManaSpendOutcome {
        self.spend_for_spell_with_penalty(mana_cost, Self::DEFAULT_HP_MULTIPLIER)
    }

    pub fn spend_for_spell_with_penalty(
        &mut self,
        mana_cost: u16,
        hp_multiplier: u8,
    ) -> ManaSpendOutcome {
        if mana_cost == 0 {
            return ManaSpendOutcome::ManaOnly { mana_spent: 0 };
        }
        let available = self.current();
        if available >= mana_cost {
            self.spend(mana_cost);
            ManaSpendOutcome::ManaOnly { mana_spent: mana_cost }
        } else if available == 0 {
            let hp_cost = mana_cost.saturating_mul(hp_multiplier as u16);
            ManaSpendOutcome::OverflowToHealth {
                mana_spent: 0,
                hp_cost,
            }
        } else {
            let remaining = mana_cost - available;
            self.current = 0;
            let hp_cost = remaining.saturating_mul(hp_multiplier as u16);
            ManaSpendOutcome::OverflowToHealth {
                mana_spent: available,
                hp_cost,
            }
        }
    }

    pub fn restore(&mut self, amount: u16) {
        let cap = self.capacity();
        let new_value = self.current().saturating_add(amount);
        self.current = new_value.min(cap);
    }

    pub fn add_bonus_capacity(&mut self, bonus: i16) {
        self.bonus_capacity = (self.bonus_capacity + bonus).clamp(-20, 40);
        self.current = self.current.min(self.capacity());
    }

    pub fn sync_focus(&mut self, is_caster: bool) {
        if self.is_caster == is_caster {
            return;
        }
        self.is_caster = is_caster;
        let new_base = Self::starting_capacity(is_caster);
        if new_base > self.base_capacity {
            self.restore(new_base - self.base_capacity);
        } else if new_base < self.base_capacity {
            let delta = self.base_capacity - new_base;
            self.current = self.current.saturating_sub(delta);
        }
        self.base_capacity = new_base;
        self.current = self.current.min(self.capacity());
    }
}

impl Default for ManaPool {
    fn default() -> Self {
        Self::new(false)
    }
}
