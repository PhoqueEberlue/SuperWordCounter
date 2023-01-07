mod splitter;
use crate::swc::splitter::{open_files, read_all_files};

mod mapper;
use crate::swc::mapper::main_mapper;

use std::{fs, thread};
use std::fs::{File};
use std::io::{BufReader};
use std::path::{Path};

const NUMBER_BYTES_SURPLUS: usize = 10;

pub fn launch_map_reduce(directory_path: &String, number_mapper: u8, number_reducer: u8) {

    // Create a Path object of our directory
    let dir_path: &Path = Path::new(directory_path.as_str());

    // Counting the number of files
    let file_count = fs::read_dir(dir_path).unwrap().count();

    // Initializing a vector to store BufReader of each files
    let mut buf_reader_vector: Vec<BufReader<File>> = Vec::with_capacity(file_count);

    // Open files
    let total_bytes: u64 = open_files(dir_path, &mut buf_reader_vector).unwrap();

    println!("----------------------------------------------------");
    println!("Total number of bytes: {}", total_bytes);
    println!("Number of mapper: {}", number_mapper);

    // Chunk size is calculated dividing the total size of the files by the number of mapper
    // Each mapper have a similar number of byte to work on
    let base_chunk_size: usize = total_bytes as usize / number_mapper as usize;

    println!("{}/{} = {} bytes for each mapper", total_bytes, number_mapper, base_chunk_size);
    println!("----------------------------------------------------");

    // Declaring a slightly bigger chunk size to prevent reallocating if the chunk would cut a word at the end
    let bigger_chunk_size = base_chunk_size + NUMBER_BYTES_SURPLUS;

    let mut chunk_vector: Vec<Vec<u8>> = read_all_files(&mut buf_reader_vector, base_chunk_size, bigger_chunk_size, NUMBER_BYTES_SURPLUS, number_mapper).unwrap();

    println!("Number chunk: {}", chunk_vector.len());

    for (i, chunk) in chunk_vector.iter().enumerate() {
        println!("Size of the chunk {}: {} bytes", i, chunk.len());
    }

    // Launching threads
    let mut handles = Vec::new();

    for i in 0..number_mapper {
        let chunk = chunk_vector.pop().unwrap();

        handles.push(thread::spawn(move || {
            main_mapper(i, chunk)
        }));
    }

    let mut i = 0;
    let mut total_words = 0;

    for handle in handles{
        let words = handle.join().unwrap().unwrap();
        println!("Thread {}: {:?} words", i, words.len());
        total_words += words.len();
        i += 1;
    }

    println!("Total words: {}", total_words);
}