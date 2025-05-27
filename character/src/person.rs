use crate::utilis::{_read_int, _read_string};

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
        println!("Enter your First Name: ");
        let first_name = _read_string();
        println!("Enter your Last Name: ");
        let last_name = _read_string();
        println!("Enter your Age: ");
        let age = _read_int();
        let gender = Gender::Male;

        Self { first_name, last_name, age, gender }
    }
}