use std::{thread};
use std::collections::{HashMap};

pub fn main_mapper(thread_number: u16, chunk: Vec<u8>, number_reducer: u16) -> thread::Result<Vec<HashMap<Vec<u8>, u16>>> {
    /*
    Main function of the mapper that should be called by the mapper thread.
    Returns a thread result containing a vector of words.
     */
    println!("Launching thread {}", thread_number);

    let mut hash_map_vector: Vec<HashMap<Vec<u8>, u16>> = Vec::with_capacity(number_reducer as usize);

    // Creating one hash map for every reducer
    for _ in 0..number_reducer {
        let hash_map: HashMap<Vec<u8>, u16> = HashMap::new();
        hash_map_vector.push(hash_map);
    }

    let mut current_word: Vec<u8> = Vec::new();

    // Loop through every char of the chunk
    for char_code in chunk {
        // If char code is not a letter
        if char_code < 65 as u8 || (char_code > 90 as u8 && char_code < 97 as u8) || char_code > 123 as u8 {
            // If current word vector is not empty
            if !current_word.is_empty() {
                // Gets the index of which reducer the current word will be provided
                let index_hash_map = word_split_function(&current_word, number_reducer);

                // Gets the hash map
                let hash_map: &mut HashMap<Vec<u8>, u16> = hash_map_vector.get_mut(index_hash_map as usize).unwrap();

                // Increment the value of the current word if it exists in the hash map, otherwise insert value 1
                hash_map.entry(current_word).and_modify(|count| *count += 1).or_insert(1);

                current_word = Vec::new();
            }
        } else {
            // Else add the current char
            current_word.push(char_code);
        }
    }

    Ok(hash_map_vector)
}

pub fn word_split_function(word: &Vec<u8>, modulus: u16) -> u16 {
    /*

     */
    let mut res: u16 = 0;

    for char in word.iter() {
        res += *char as u16;
    }

    res % modulus
}