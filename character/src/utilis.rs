use std::io::*;

const ERR_MSG: &str = "Error reading user input";
const INVALID_OPTION: &str = "Invalid input";

pub fn _read_string() -> String {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res)
    .expect(ERR_MSG);
    user_res
    .trim()
    .to_string()
}

pub fn _read_int() -> i32 {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res)
    .expect(ERR_MSG);
    user_res
    .trim()
    .parse::<i32>()
    .expect(INVALID_OPTION)
}

pub fn _read_float() -> f64 {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res)
    .expect(ERR_MSG);
    user_res
    .trim()
    .parse::<f64>().expect(INVALID_OPTION)
}