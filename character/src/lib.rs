use core::result::Result::Ok;
use std::str::FromStr;
use std::io::*;
use crate::warrior::warrior::WarriorInfo;

use utilis::{_read_float, _read_int, _read_string};

mod warrior;
mod utilis;

#[derive(Debug)]
enum VitalityElement {
    Fire,
    Water,
    Ice,
    Earth,
    Plants,
    Air,
    Lighting,
    Noul,
    Null,
}

#[derive(Debug)]
enum SkinColour {
    Black,
    White,
    Yellow,
    Red,
}

#[derive(Debug)]
enum Nation {
    Arigo,
}

#[derive(Debug)]
enum SocialClass {
    Royal,
    Noble,
    Military,
    Civilian,
}

#[derive(Debug)]
enum WarriorClass {
    Buffer,
    DeBuffer,
    Healer,
    MainDamageDealer,
    SubDamageDealer,
}

#[derive(Debug)]
pub struct Character {
    first_name: String,
    last_name: String,
    age: i32,
    element: VitalityElement,
    height: f64,
    weight: f64,
    skin: SkinColour,
    nationality: Nation,
    social_class: SocialClass,       
    warrior_class: WarriorInfo,
}

impl Character {
    pub fn new() -> Self {
        println!("Enter your First Name: ");
        let first_name = _read_string();
        println!("Enter your Last Name: ");
        let last_name = _read_string();
        println!("Enter your Age: ");
        let age = _read_int();
        Self::VitalityList();
        let element = match _read_int() {
            1 => VitalityElement::Air,
            2 => VitalityElement::Earth,
            3 => VitalityElement::Fire,
            4 => VitalityElement::Ice,
            5 => VitalityElement::Lighting,
            6 => VitalityElement::Noul,
            7 => VitalityElement::Plants,
            8 => VitalityElement::Water,
            _ => VitalityElement::Null,
        };
        println!("Enter your Height: ");
        let height = _read_float();
        println!("Enter your Weight: ");
        let weight = _read_float();
        Self::SkinColourList();
        let skin = match _read_int() {
            1 => SkinColour::Black,
            2 => SkinColour::Red,
            3 => SkinColour::White,
            4 => SkinColour::Yellow,
            _ => SkinColour::Black,
        };
        let nationality = Nation::Arigo;
        let social_class = SocialClass::Military;
        let warrior_class= WarriorInfo::new();

        Character{ 
            first_name,
            last_name,
            age,
            element,
            height,
            weight,
            skin,
            nationality,
            social_class,       
            warrior_class,
        }
   }
   pub fn ShowCharacterProfile(character: Character) {
        println!("{:?}", character);
   }
   fn VitalityList() {
        println!("Vitality Element");
        println!("1. Air");
        println!("2. Earth");
        println!("3. Fire");
        println!("4. Ice");
        println!("5. Lighting");
        println!("6. Noul");
        println!("7. Plants");
        println!("8. Water");
        println!("Enter the number of your Vitality Element: ");
   }
   fn SkinColourList() {
        println!("Skin Colour");
        println!("1. Black");
        println!("2. Red");
        println!("3. White");
        println!("4. Yellow");
        println!("Enter the number of your Skin Colour: ");
   }
}