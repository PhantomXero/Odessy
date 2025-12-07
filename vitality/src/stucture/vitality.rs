use crate::stucture::spell::Spell;
use el_roi::read_int;

#[derive(Debug, Clone)]
enum VitalityElement {
    Venta,
    Terra,
    Igna,
    Glacia,
    Fulga,
    Planta,
    Aqua,
    Nulla,
}

#[derive(Debug, Clone)]

pub enum VitalityLevel {
    Dormant {
        control: i32,
        modifier: i32,
        power: i32,
    },
    Awakened {
        control: i32,
        modifier: i32,
        power: i32,
    },
    Attuned {
        control: i32,
        modifier: i32,
        power: i32,
    },
    Channeling {
        control: i32,
        modifier: i32,
        power: i32,
    },
    Empowered {
        control: i32,
        modifier: i32,
        power: i32,
    },
    Mastered {
        control: i32,
        modifier: i32,
        power: i32,
    },
    Ghost {
        control: i32,
        modifier: i32,
        power: i32,
    },
}

#[derive(Debug, Clone)]
pub struct Vitality {
    vitality_type: VitalityElement,
    level: VitalityLevel,
    modifier: i32,
    spells: Vec<Spell>,
    control: i32,
}

trait Control {
    fn new() -> Self;
    fn edit_vitality_level(&mut self, con: i32, modi: i32, pwr: i32);
    fn edit_vitality_modifier(&self) -> Self;
    fn add_vitality_spell(&self) -> Self;
    fn level_up(&mut self);
    fn list();
}

impl Vitality {
    fn new() -> Self {
        Self::list();
        let vitality_type = match read_int("Enter the number of your Vitality Element: ") {
            1 => VitalityElement::Venta,
            2 => VitalityElement::Terra,
            3 => VitalityElement::Igna,
            4 => VitalityElement::Glacia,
            5 => VitalityElement::Fulga,
            7 => VitalityElement::Planta,
            8 => VitalityElement::Aqua,
            _ => VitalityElement::Nulla,
        };
        let level = VitalityLevel::Dormant {
            control: 1,
            modifier: 1,
            power: 1,
        };
        let modifier = 0;
        let spells = Vec::new();
        let control = 0;

        Self {
            vitality_type,
            level,
            modifier,
            spells,
            control,
        }
    }

    // fn edit_vitality_level(&mut self, con: i32, modi: i32, pwr: i32) {
    //     let max = 10;
    //     let mut new_vitality = self.clone();

    //     new_vitality.level = match &self.level {
    //         VitalityLevel::Dormant { control, modifier, power } => {
    //             if (con == max && pwr == max) {
    //                 self.level_up();
    //             } else {
    //                 self.level = VitalityLevel::Dormant {
    //                     control: if *control < max { *control + con } else { *control },
    //                     modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //                     power: if *power < max { *power + pwr } else { *power },
    //                 };
    //             }
    //         },
    //         VitalityLevel::Awakened { control, modifier, power } => {
    //             if (*control == max && *power == max) {
    //                 self.level_up()
    //             } else {
    //                 VitalityLevel::Awakened {
    //                     control: if *control < max { *control + con } else { *control },
    //                     modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //                     power: if *power < max { *power + pwr } else { *power },
    //                 }
    //             }
    //         },
    //         VitalityLevel::Attuned { control, modifier, power } => {
    //             if (*control == max && *power == max) {
    //                 self.level_up()
    //             } else {
    //                 VitalityLevel::Attuned {
    //                     control: if *control < max { *control + con } else { *control },
    //                     modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //                     power: if *power < max { *power + pwr } else { *power },
    //                 }
    //             }
    //         },
    //         VitalityLevel::Channeling { control, modifier, power } => VitalityLevel::Channeling {
    //             control: if *control < max { *control + con } else { *control },
    //             modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //             power: if *power < max { *power + pwr } else { *power },
    //         },
    //         VitalityLevel::Empowered { control, modifier, power } => VitalityLevel::Empowered {
    //             control: if *control < max { *control + con } else { *control },
    //             modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //             power: if *power < max { *power + pwr } else { *power },
    //         },
    //         VitalityLevel::Mastered { control, modifier, power } => VitalityLevel::Mastered {
    //             control: if *control < max { *control + con } else { *control },
    //             modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //             power: if *power < max { *power + pwr } else { *power },
    //         },
    //         VitalityLevel::Ghost { control, modifier, power } => {
    //             if (*control == max && *power == max) {
    //                 self.level_up()
    //             } else {
    //                 VitalityLevel::Ghost {
    //                     control: if *control < max { *control + con } else { *control },
    //                     modifier: if *modifier < max { *modifier + modi } else { *modifier },
    //                     power: if *power < max { *power + pwr } else { *power },
    //                 }
    //             }
    //         },
    //     };

    // }

    fn edit_vitality_modifier(&self) -> Self {
        let mut new_vitality = self.clone();
        new_vitality.modifier += 1; // Example modification
        new_vitality
    }

    fn add_vitality_spell(&self) -> Self {
        // Logic to edit vitality properties
        let mut new_vitality = self.clone();
        new_vitality.spells.push(Spell::new()); // Example modification
        new_vitality
    }

    fn level_up(&mut self) {
        match self.level {
            VitalityLevel::Dormant { .. } => {
                self.level = VitalityLevel::Awakened {
                    control: 1,
                    modifier: 1,
                    power: 1,
                }
            }
            VitalityLevel::Awakened { .. } => {
                self.level = VitalityLevel::Attuned {
                    control: 1,
                    modifier: 1,
                    power: 1,
                }
            }
            VitalityLevel::Attuned { .. } => {
                self.level = VitalityLevel::Channeling {
                    control: 1,
                    modifier: 1,
                    power: 1,
                }
            }
            VitalityLevel::Channeling { .. } => {
                self.level = VitalityLevel::Empowered {
                    control: 1,
                    modifier: 1,
                    power: 1,
                }
            }
            VitalityLevel::Empowered { .. } => {
                self.level = VitalityLevel::Mastered {
                    control: 1,
                    modifier: 1,
                    power: 1,
                }
            }
            _ => {
                self.level = VitalityLevel::Ghost {
                    control: 1,
                    modifier: 1,
                    power: 1,
                }
            }
        }
    }

    fn list() {
        println!("Vitality Element");
        println!("1. {:?}", VitalityElement::Venta);
        println!("2. {:?}", VitalityElement::Terra);
        println!("3. {:?}", VitalityElement::Igna);
        println!("4. {:?}", VitalityElement::Glacia);
        println!("5. {:?}", VitalityElement::Fulga);
        println!("6. {:?}", VitalityElement::Nulla);
        println!("7. {:?}", VitalityElement::Planta);
        println!("8. {:?}", VitalityElement::Aqua);
    }
}
