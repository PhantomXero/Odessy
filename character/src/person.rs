use el_roi::{read_int, read_string};
use serde::{Deserialize, Serialize};
use std::fmt;

const IDENTITY_GUIDE: &str = "Identity covers how NPCs will address you and how save slots are labelled. You can tweak these values anytime if the story changes.";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl Gender {
    fn prompt_selection() -> Self {
        println!("Select Gender");
        println!("1. Male");
        println!("2. Female");
        match read_int("Enter the number of your Gender: ") {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::default(),
        }
    }
    fn as_label(&self) -> &'static str {
        match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_label())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    first_name: String,
    last_name: String,
    age: i32,
    gender: Gender,
}

impl PersonalInfo {
    pub fn new(first_name: String, last_name: String, age: i32, gender: Gender) -> Self {
        Self {
            first_name,
            last_name,
            age,
            gender,
        }
    }

    pub fn from_prompt() -> Self {
        let first_name = read_string("Enter your First Name: ");
        let last_name = read_string("Enter your Last Name: ");
        let age = read_int("Enter your Age: ");
        let gender = Gender::prompt_selection();
        Self::new(first_name, last_name, age, gender)
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    pub fn age(&self) -> i32 {
        self.age
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
            .trim()
            .to_string()
    }

    pub fn edit_identity_section(&mut self) {
        println!("--- Identity ---");
        println!("{}", IDENTITY_GUIDE);

        if prompt_yes_no("Update first name? (1 Yes / 2 No)") {
            self.first_name = read_string("Enter new First Name: ");
        }

        if prompt_yes_no("Update last name? (1 Yes / 2 No)") {
            self.last_name = read_string("Enter new Last Name: ");
        }

        if prompt_yes_no("Update age? (1 Yes / 2 No)") {
            self.age = read_int("Enter new Age: ");
        }

        if prompt_yes_no("Update gender? (1 Yes / 2 No)") {
            self.gender = Gender::prompt_selection();
        }
    }
}

impl Default for PersonalInfo {
    fn default() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            age: 0,
            gender: Gender::default(),
        }
    }
}

impl fmt::Display for PersonalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name   : {}", self.full_name())?;
        writeln!(f, "Age    : {}", self.age)?;
        write!(f, "Gender : {}", self.gender)
    }
}

fn prompt_yes_no(message: &str) -> bool {
    println!("{}", message);
    matches!(read_int("Selection: "), 1)
}
