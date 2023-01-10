mod swc;

use std::env;
use crate::swc::launch_map_reduce;

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1];
    launch_map_reduce(directory_path, 8, 8, true);
}

//TODO 1: Add command line arguments support
//TODO 2: Test some benchmark tools, maybe test with different number of mappers / reducers

//TODO<optional>: Make some distributions graph with different modulus functions

//TODO<later>: Implement a socket version
