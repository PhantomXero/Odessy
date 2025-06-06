use super::fightstyle::FightingStyleInfo;
use super::weapon::WeaponInfo;

#[derive(Debug)]
enum WarriorClass {
    Buffer,
    DeBuffer,
    Healer,
    MainDamageDealer,
    SubDamageDealer,
}

#[derive(Debug)]
enum WarriorRank {
    UnRanked,
    Novice,
    Amateur,
    Intermediate,
    Bronze,
    Sliver,
    Gold,
}

#[derive(Debug)]
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


        Self{main_class, sub_class, main_fighting_style, sub_fighting_style, main_weapon, rank}
    }
    pub fn level_up(&mut self) {
        match self.rank {
            WarriorRank::UnRanked => self.rank = WarriorRank::Novice,
            WarriorRank::Novice => self.rank = WarriorRank::Amateur,
            WarriorRank::Amateur => self.rank = WarriorRank::Intermediate,
            WarriorRank::Intermediate => self.rank = WarriorRank::Bronze,
            WarriorRank::Bronze => self.rank = WarriorRank::Sliver,
            WarriorRank::Gold => self.rank = WarriorRank::Gold,
            _ => self.rank = WarriorRank::UnRanked,
        };
    }
    pub fn edit(&mut self) {

    } 
}