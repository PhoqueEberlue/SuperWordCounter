mod swc;

use std::env;
use crate::swc::launch_map_reduce;

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1];
    launch_map_reduce(directory_path, 8, 8, true);
}
