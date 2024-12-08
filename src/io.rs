use std::{env, fs};
use std::process::exit;

pub fn read_input() -> String{
    let input_file = format!("input/{}.txt", env::current_exe().unwrap().file_name().unwrap().to_str().unwrap());
    match fs::read_to_string(&input_file) {
        Ok(input) => {
            input
        },
        Err(e) => {
            println!("Error reading file {}: {}", input_file, e);
            exit(1);
        },
    }
}