use std::{thread};

pub fn main_mapper(thread_number: u8, chunk: Vec<u8>) -> thread::Result<Vec<Vec<u8>>> {
    /*
    Main function of the mapper that should be called by the mapper thread.
    Returns a thread result containing a vector of words.
     */
    println!("Launching thread {}", thread_number);

    let mut word_vec: Vec<Vec<u8>> = Vec::new();
    let mut current_word: Vec<u8> = Vec::new();

    for char_code in chunk {
        // If char code is equal to space or carriage return and current word is not empty
        if (char_code == 32 as u8 || char_code == 13 as u8) && !current_word.is_empty(){
            // Adds the current word the the word vector
            word_vec.push(current_word);
            current_word = Vec::new();
        } else {
            // Else add the current char
            current_word.push(char_code);
        }
    }

    Ok(word_vec)
}

