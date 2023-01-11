extern crate core;

mod swc;

use std::env;
use std::process::exit;
use crate::swc::launch_map_reduce;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default arguments
    let mut directory_path: &String = &String::from("./");
    let mut number_mapper: u16 = 8;
    let mut number_reducer: u16 = 8;
    let mut to_lower: bool = true;
    let mut modulus_function: String = String::from("len");

    let mut iterator = args.iter();

    loop {
        let current_arg = match iterator.next() {
            Some(res) => res,
            None => break
        };

        // Matching arguments
        match current_arg.as_str() {
            "-i" | "--input-folder" => directory_path = iterator.next().unwrap(),
            "-m" | "--mapper" => number_mapper = iterator.next().unwrap().parse::<u16>().unwrap(),
            "-r" | "--reducer" => number_reducer = iterator.next().unwrap().parse::<u16>().unwrap(),
            "-l" | "--to-lower" => to_lower = true,
            "-f" | "--modulus-function" => modulus_function = match iterator.next().unwrap().as_str() {
                "len" => String::from("len"),
                "ascii" => String::from("ascii"),
                _ => {
                    println!("Wrong argument for --modulus-function. Possible arguments are `len`, `ascii`");
                    exit(1)
                }
            },
            _ => {}
        }
    }

    launch_map_reduce(directory_path, number_mapper, number_reducer, to_lower, modulus_function);
}

//TODO: Test some benchmark tools, maybe test with different number of mappers / reducers

//TODO<optional>: Make some distributions graph with different modulus functions

//TODO<later>: Implement a socket version
