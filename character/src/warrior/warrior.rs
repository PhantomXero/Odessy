use super::fightstyle::FightingStyleInfo;
use super::weapon::WeaponInfo;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum WarriorClass {
    Buffer,
    DeBuffer,
    Healer,
    MainDamageDealer,
    SubDamageDealer,
}

impl fmt::Display for WarriorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WarriorClass::Buffer => "Support",
            WarriorClass::DeBuffer => "Breaker",
            WarriorClass::Healer => "Healer",
            WarriorClass::MainDamageDealer => "Vanguard DPS",
            WarriorClass::SubDamageDealer => "Skirmisher",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum WarriorRank {
    UnRanked,
    Novice,
    Amateur,
    Intermediate,
    Bronze,
    Sliver,
    Gold,
}

impl fmt::Display for WarriorRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            WarriorRank::UnRanked => "Unranked",
            WarriorRank::Novice => "Novice",
            WarriorRank::Amateur => "Amateur",
            WarriorRank::Intermediate => "Intermediate",
            WarriorRank::Bronze => "Bronze",
            WarriorRank::Sliver => "Silver",
            WarriorRank::Gold => "Gold",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarriorInfo {
    main_class: Option<WarriorClass>,
    sub_class: Option<Vec<WarriorClass>>,
    main_fighting_style: Option<FightingStyleInfo>,
    sub_fighting_style: Option<Vec<FightingStyleInfo>>,
    main_weapon: WeaponInfo,
    rank: WarriorRank,
}

impl WarriorInfo {
    pub fn new() -> Self {
        let main_class = None;
        let sub_class = None;
        let main_fighting_style = None;
        let sub_fighting_style = None;
        let main_weapon = WeaponInfo::new();
        let rank = WarriorRank::UnRanked;

        Self {
            main_class,
            sub_class,
            main_fighting_style,
            sub_fighting_style,
            main_weapon,
            rank,
        }
    }
    pub fn level_up(&mut self) {
        match self.rank {
            WarriorRank::UnRanked => self.rank = WarriorRank::Novice,
            WarriorRank::Novice => self.rank = WarriorRank::Amateur,
            WarriorRank::Amateur => self.rank = WarriorRank::Intermediate,
            WarriorRank::Intermediate => self.rank = WarriorRank::Bronze,
            WarriorRank::Bronze => self.rank = WarriorRank::Sliver,
            WarriorRank::Sliver => self.rank = WarriorRank::Gold,
            _ => self.rank = WarriorRank::Gold,
        };
    }
    pub fn edit(&mut self) {}
}

impl Default for WarriorInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WarriorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let main_class = self
            .main_class
            .as_ref()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "Unassigned".to_string());
        let sub_classes = self
            .sub_class
            .as_ref()
            .map(|list| {
                list.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "None".to_string());
        let fighting_style = self
            .main_fighting_style
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Style: Unassigned | Weapons: None".to_string());
        write!(
            f,
            "Rank: {}\nMain Class: {}\nSub Classes: {}\n{}\nWeapon: {}",
            self.rank, main_class, sub_classes, fighting_style, self.main_weapon
        )
    }
}
