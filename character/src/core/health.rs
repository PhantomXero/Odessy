use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthPool {
    current: u8,
    max: u8,
}

impl HealthPool {
    pub const CAP: u8 = 100;
    const BASELINE: u8 = 90;
    const MIN_BONUS: u8 = 0;
    const MAX_BONUS: u8 = 10;

    pub fn new() -> Self {
        Self::from_bonus(0)
    }

    pub fn from_bonus(bonus: u8) -> Self {
        let max = Self::calculate_max(bonus);
        Self { current: max, max }
    }

    fn calculate_max(bonus: u8) -> u8 {
        let clamped = bonus.clamp(Self::MIN_BONUS, Self::MAX_BONUS);
        (Self::BASELINE + clamped).min(Self::CAP)
    }

    pub fn apply_bonus(&mut self, bonus: u8) {
        let max = Self::calculate_max(bonus);
        self.max = max;
        self.current = max;
    }

    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn max(&self) -> u8 {
        self.max
    }

    pub fn spend(&mut self, amount: u16) -> bool {
        if amount == 0 {
            return true;
        }
        let current_u16 = u16::from(self.current);
        if amount >= current_u16 {
            self.current = 0;
            false
        } else {
            let spend = amount as u8;
            self.current = self.current.saturating_sub(spend);
            true
        }
    }

    pub fn apply_spell_penalty(&mut self, hp_cost: u16) -> bool {
        self.spend(hp_cost)
    }
}

impl Default for HealthPool {
    fn default() -> Self {
        Self::new()
    }
}
