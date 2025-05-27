use super::utilis::_read_int;

enum List {
    Nation,
    SocialClass,
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
pub struct CivicInfo {
    nationality: Nation,
    social_class: SocialClass,
}

impl CivicInfo {
    pub fn new() -> Self {
        let nationality = Nation::Arigo;
        let social_class = SocialClass::Civilian;

        Self{nationality, social_class}
    }
    pub fn edit(&mut self) {
        Self::lists(List::Nation);
        self.nationality = match  _read_int() {
            1 => Nation::Arigo,
            _ => Nation::Arigo,
        };
        Self::lists(List::SocialClass);
        self.social_class = match  _read_int() {
            1 => SocialClass::Civilian,
            2 => SocialClass::Military,
            3 => SocialClass::Noble,
            4 => SocialClass::Royal,
            _ => SocialClass::Civilian,
        };
    }
    pub fn level_up(&mut self) {
        match self.social_class {
            SocialClass::Civilian => self.social_class = SocialClass::Military,
            SocialClass::Military => self.social_class = SocialClass::Noble,
            SocialClass::Noble => self.social_class = SocialClass::Royal,
            _ => self.social_class = SocialClass::Civilian,
        }
    }
    pub fn lists(list: List) {
        match list {
            List::Nation => {
                println!("Nation");
                println!("1. {:?}", Nation::Arigo);
                println!("Enter the number of your Nationality: ");
            },
            List::SocialClass => {
                println!("Social Class");
                println!("1. {:?}", SocialClass::Civilian);
                println!("2. {:?}", SocialClass::Military);
                println!("3. {:?}", SocialClass::Noble);
                println!("4. {:?}", SocialClass::Royal);
                println!("Enter the number of your Social Class: ");
            },
        }
    }
}