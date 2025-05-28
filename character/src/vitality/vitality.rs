use crate::utilis::_read_int;

#[derive(Debug)]
enum VitalityLevel {
    Dormant,
    Awakened,
    Attuded,
    Channeling,
    Empowered,
    Mastered,
    Ghost,
}
#[derive(Debug)]
enum VitalityElement {
    Fire,
    Water,
    Ice,
    Earth,
    Plants,
    Air,
    Lighting,
    Void,
    Null,
}

#[derive(Debug)]
pub struct Vitalityinfo {
    element: VitalityElement,
    level: VitalityLevel
}

impl Vitalityinfo {
    pub fn new() -> Self {
        Self::list();
        let element = match _read_int() {
            1 => VitalityElement::Air,
            2 => VitalityElement::Earth,
            3 => VitalityElement::Fire,
            4 => VitalityElement::Ice,
            5 => VitalityElement::Lighting,
            6 => VitalityElement::Void,
            7 => VitalityElement::Plants,
            8 => VitalityElement::Water,
            _ => VitalityElement::Null,
        };
        let level = VitalityLevel::Dormant;

        Self{element, level}
    }
    pub fn level_up(&mut self) {
        match self.level {
            VitalityLevel::Dormant => self.level = VitalityLevel::Awakened,
            VitalityLevel::Awakened => self.level = VitalityLevel::Attuded,
            VitalityLevel::Attuded => self.level = VitalityLevel::Channeling,
            VitalityLevel::Channeling => self.level = VitalityLevel::Empowered,
            VitalityLevel::Empowered => self.level = VitalityLevel::Mastered,
            _ => self.level = VitalityLevel::Ghost
        }
    }
    pub fn list() {
        println!("Vitality Element");
        println!("1. {:?}", VitalityElement::Air);
        println!("2. {:?}", VitalityElement::Earth);
        println!("3. {:?}", VitalityElement::Fire);
        println!("4. {:?}", VitalityElement::Ice);
        println!("5. {:?}", VitalityElement::Lighting);
        println!("6. {:?}", VitalityElement::Void);
        println!("7. {:?}", VitalityElement::Plants);
        println!("8. {:?}", VitalityElement::Water);
        println!("Enter the number of your Vitality Element: ");
    }
}