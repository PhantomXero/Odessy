use std::io::*;
const ERR_MSG: &str = "Error reading user input";

pub fn _read() -> String {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res)
    .expect(ERR_MSG);
    user_res
}