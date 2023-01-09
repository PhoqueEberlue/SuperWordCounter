mod splitter;
mod mapper;
mod reducer;

use std::{fs, thread};
use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufReader};
use std::path::{Path};

const NUMBER_BYTES_SURPLUS: usize = 10;

pub fn launch_map_reduce(directory_path: &String, number_mapper: u16, number_reducer: u16, to_lower: bool) {

    // Create a Path object of our directory
    let dir_path: &Path = Path::new(directory_path.as_str());

    // Counting the number of files
    let file_count = fs::read_dir(dir_path).unwrap().count();

    // Initializing a vector to store BufReader of each files
    let mut buf_reader_vector: Vec<BufReader<File>> = Vec::with_capacity(file_count);

    // Open files
    let total_bytes: u64 = splitter::open_files(dir_path, &mut buf_reader_vector).unwrap();

    #[cfg(debug_assertions)]
    {
        println!("----------------------------------------------------");
        println!("Total number of bytes: {}", total_bytes);
        println!("Number of mapper: {}", number_mapper);
    }

    // Chunk size is calculated dividing the total size of the files by the number of mapper
    // Each mapper have a similar number of byte to work on
    let base_chunk_size: usize = total_bytes as usize / number_mapper as usize;

    #[cfg(debug_assertions)]
    {
        println!("{}/{} = {} bytes for each mapper", total_bytes, number_mapper, base_chunk_size);
        println!("----------------------------------------------------");
    }

    // Declaring a slightly bigger chunk size to prevent reallocating if the chunk would cut a word at the end
    let bigger_chunk_size = base_chunk_size + NUMBER_BYTES_SURPLUS;

    let mut chunk_vector: Vec<Vec<u8>> = splitter::read_all_files(&mut buf_reader_vector, base_chunk_size, bigger_chunk_size, NUMBER_BYTES_SURPLUS, number_mapper).unwrap();

    #[cfg(debug_assertions)]
    {
        println!("Number chunk: {}", chunk_vector.len());

        for (i, chunk) in chunk_vector.iter().enumerate() {
            println!("Size of the chunk {}: {} bytes", i, chunk.len());
        }
    }

    // Defining thread pool
    let mut handles_mapper = Vec::with_capacity(number_mapper as usize);

    // Launching mapper threads
    for i in 0..number_mapper {
        let chunk = chunk_vector.pop().unwrap();

        handles_mapper.push(thread::spawn(move || {
            // Calling main mapper function + implicit return of Thread result
            mapper::map(i, chunk, number_reducer, to_lower)
        }));
    }

    let mut len_hash_map_vec: Vec<u32> = Vec::with_capacity(number_reducer as usize);
    for _ in 0..number_reducer {
        len_hash_map_vec.push(0);
    }

    let mut hash_map_vector_vector: Vec<Vec<HashMap<Vec<u8>, u64>>> = Vec::with_capacity(number_reducer as usize);

    for handle in handles_mapper {
        let hash_map_vector = handle.join().unwrap().unwrap();

        hash_map_vector_vector.push(hash_map_vector);
    }


    #[cfg(debug_assertions)]
    for (i, len_hash_map) in len_hash_map_vec.iter().enumerate() {
        println!("Reducer {} will receive {} words", i, len_hash_map);
    }

    let mut handles_reducer = Vec::with_capacity(number_reducer as usize);

    // Launching reducer threads
    for _ in 0..number_reducer {
        let mut reducer_hash_map_vector: Vec<HashMap<Vec<u8>, u64>> = Vec::with_capacity(number_reducer as usize);

        for i in 0..number_reducer {
            let hash_map_vector: &mut Vec<HashMap<Vec<u8>, u64>> = hash_map_vector_vector.get_mut(i as usize).unwrap();

            reducer_hash_map_vector.push(hash_map_vector.pop().unwrap());
        }

        handles_reducer.push(thread::spawn(move || {
            // Calling main mapper function + implicit return of Thread result
            reducer::reduce(reducer_hash_map_vector)
        }));
    }

    let mut hash_map_vector: Vec<HashMap<Vec<u8>, u64>> = Vec::with_capacity(number_reducer as usize);
    for handle in handles_reducer {
        let hash_map = handle.join().unwrap().unwrap();

        hash_map_vector.push(hash_map);
    }

    let mut hash_map_final = hash_map_vector.pop().unwrap();

    for hash_map in hash_map_vector.iter_mut() {
        for (k, v) in hash_map.drain().into_iter() {
            hash_map_final.insert(k, v);
        }
    }

    #[cfg(debug_assertions)]
    for val in hash_map_final.keys() {
        let v = hash_map_final.get(val).unwrap();

        if *v > 1000 {
            println!("{:?}: {}", String::from_utf8(val.clone()).unwrap(), v);
        }
    }
}