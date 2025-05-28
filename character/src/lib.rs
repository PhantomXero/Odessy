use crate::warrior::warrior::WarriorInfo;

use nation::CivicInfo;
use person::PersonalInfo;
use crate::physical::physical::PhysicalInfo;
use utilis::{_read_float, _read_int, _read_string};
use crate::vitality::vitality::Vitalityinfo;

mod nation;
mod person;
mod physical;
mod warrior;
mod utilis;
mod vitality;

#[derive(Debug)]
pub struct Character {
    personal: PersonalInfo,
    vitality: Vitalityinfo,
    physical: PhysicalInfo,
    civic: CivicInfo,      
    warrior: WarriorInfo,
}

impl Character {
    pub fn new() -> Self {
        let personal = PersonalInfo::new();
        let vitality = Vitalityinfo::new();
        println!("Enter your Height: ");
        let physical = PhysicalInfo::new();
        let civic = CivicInfo::new();
        let warrior= WarriorInfo::new();
        Self{ 
            personal,
            vitality,
            physical,
            civic,
            warrior,
        }
   }
   pub fn ShowCharacterProfile(character: &mut Character) {
        println!("{:?}", character);
   }
   pub fn level_up(&mut self) {
        self.vitality.level_up();
        self.physical.level_up();
        self.civic.level_up();
        self.warrior.level_up();
   }
}