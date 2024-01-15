#![allow(special_module_name)]

use std::env;
use std::process;
mod utils;
mod lib;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    let ignore_case: bool = match args[3].as_str() {
        "--" => match args[4].as_str() {
            "ignore_case" => true,
            _ => false
        },
        "--ignore_case" => true,
        _ => false
    };
    

    let inputs = utils::Input::build(env::args(),ignore_case).unwrap_or_else(|error|{
        println!("problem parsing the argument: {}",error);
        process::exit(1)
    });

    if let Ok(contents) = utils::read_data(&inputs) {
        if inputs.ignore_case == true {
            for output in lib::search_insensitive(&inputs.query,&contents) {
                println!("output: {}",output)
            }
        } else {
            for output in lib::search(&inputs.query,&contents) {
                println!("output: {}",output)
            }
        }
    }
    
}
