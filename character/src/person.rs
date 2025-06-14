use el_roi::{read_int, read_string};

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub struct PersonalInfo {
    first_name: String,
    last_name: String,
    age: i32,
    gender: Gender,
}

impl PersonalInfo {
    pub fn new() -> Self {
        let first_name = read_string("Enter your First Name: ");
        let last_name = read_string("Enter your Last Name: ");
        let age = read_int("Enter your Age: ");
        let gender = Gender::Male;

        Self {
            first_name,
            last_name,
            age,
            gender,
        }
    }
}
