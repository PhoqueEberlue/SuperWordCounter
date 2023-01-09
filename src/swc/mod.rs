mod splitter;
mod mapper;
mod reducer;

use std::{fs, thread};
use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufReader};
use std::path::{Path};


pub fn launch_map_reduce(directory_path: &String, number_mapper: u16, number_reducer: u16, to_lower: bool) {

    let mut chunk_vector = splitter::split_files_from_path(directory_path, number_mapper).unwrap();

    let mut hash_map_vector_vector: Vec<Vec<HashMap<Vec<u8>, u64>>> = mapper::start_mapper_threads(chunk_vector, number_mapper, number_reducer, to_lower).unwrap();

    let hash_map: HashMap<Vec<u8>, u64> = reducer::launch_reducer_threads(hash_map_vector_vector, number_reducer).unwrap();

}