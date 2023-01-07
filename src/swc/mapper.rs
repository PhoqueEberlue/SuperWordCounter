use std::{thread};

pub fn main_mapper(thread_number: u8, chunk: Vec<u8>) -> thread::Result<Vec<Vec<u8>>> {
    println!("Launching thread {}", thread_number);

    let mut word_vec: Vec<Vec<u8>> = Vec::new();
    let mut current_word: Vec<u8> = Vec::new();

    for char_code in chunk {
        // if char code is equal to space or carriage return
        if char_code == 32 as u8 || char_code == 13 as u8 {
            word_vec.push(current_word);
            current_word = Vec::new();
        } else if !word_vec.is_empty() {
            current_word.push(char_code);
        }
    }

    Ok(word_vec)
}

