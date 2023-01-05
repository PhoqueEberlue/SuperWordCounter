use std::fs;
use std::fs::{File};
use std::io::{Read, BufRead, BufReader};
use std::path::{Path, PathBuf};

const NUMBER_REDUCER: u8 = 10;
const NUMBER_MAPPER: u8 = 16;
const NUMBER_BYTES_SURPLUS: usize = 10;

fn main() {
    // Create a Path object of our directory
    let dir_path: &Path = Path::new("./input");

    // Counting the number of files
    let file_count = fs::read_dir(dir_path).unwrap().count();

    // Initializing a vector to store BufReader of each files
    let mut buf_reader_vector: Vec<BufReader<File>> = Vec::with_capacity(file_count);

    // Open files
    let total_bytes: u64 = open_files(dir_path, &mut buf_reader_vector).unwrap();

    println!("----------------------------------------------------");
    println!("Total number of bytes: {}", total_bytes);
    println!("Number of mapper: {}", NUMBER_MAPPER);

    // Chunk size is calculated dividing the total size of the files by the number of mapper
    // Each mapper have a similar number of byte to work on
    let base_chunk_size: usize = total_bytes as usize / NUMBER_MAPPER as usize;

    println!("{}/{} = {} bytes for each mapper", total_bytes, NUMBER_MAPPER, base_chunk_size);
    println!("----------------------------------------------------");

    // Declaring a slightly bigger chunk size to prevent reallocating if the chunk would cut a word at the end
    let bigger_chunk_size = base_chunk_size + NUMBER_BYTES_SURPLUS;

    let chunk_vector: Vec<Vec<u8>> = read_all_files(&mut buf_reader_vector, base_chunk_size, bigger_chunk_size).unwrap();
}

fn open_files(dir_path: &Path, vec_file: &mut Vec<BufReader<File>>) -> std::io::Result<u64> {
    /*
    Open every file in the provided path and fill the vector with bufReader for each file.
    Returns the total length of every files in bytes.
     */
    let paths = fs::read_dir(dir_path)?;
    let mut total_bytes: u64 = 0;

    for path in paths {
        let path: PathBuf = path?.path();
        let path: &Path = path.as_path();

        let file = File::open(path)?;
        total_bytes += file.metadata()?.len();
        vec_file.push(BufReader::new(file));
    }

    Ok(total_bytes)
}

fn read_n_bytes(buf_reader: &mut BufReader<File>, chunk: &mut Vec<u8>, chunk_size: u64) -> std::io::Result<usize> {
    /*
    Reads n bytes and stops only at space or return characters.
    This function helps to split large text files without cutting words.
    Returns the number of bytes read and the chunk.
     */

    let mut nb_byte_read = buf_reader.take(chunk_size).read_to_end(chunk)?;
    //println!("Res: {:?}", String::from_utf8(chunk).unwrap());

    nb_byte_read += buf_reader.read_until(b' ', chunk)?;

    if nb_byte_read > (chunk_size as usize) + NUMBER_BYTES_SURPLUS {
        println!("Warning, chunk got reallocated: {}", nb_byte_read - chunk_size as usize);
    }
    //println!("Res2: {:?}", String::from_utf8(chunk).unwrap());

    Ok(nb_byte_read)
}


fn read_all_files(buf_reader_vector: &mut Vec<BufReader<File>>, base_chunk_size: usize, bigger_chunk_size: usize) -> std::io::Result<Vec<Vec<u8>>> {
    /*
    Read all files in buf_reader_vector and fill as many chunks as there are mapper.
    Returns a vector containing the chunks. Each chunk is a vector of u8.
     */

    // Creating a vector to store each chunk that will be provided to the mappers
    let mut chunk_vector: Vec<Vec<u8>> = Vec::with_capacity(NUMBER_MAPPER as usize);

    // Tracking the chunk index for debug purposes
    let mut chunk_index: u8 = 0;
    // Declaring the first chunk
    let mut chunk: Vec<u8> = Vec::with_capacity(bigger_chunk_size as usize);

    // Byte read counter
    let mut nb_byte_read: usize = 0;

    // Tracking the BufReader index for debug purposes
    let mut buf_index: u8 = 0;
    let mut buf_reader: BufReader<File> = buf_reader_vector.pop().unwrap();

    // While there are BufReaders remaining
    while !buf_reader_vector.is_empty() {
        println!("File number: {}", buf_index);
        println!("Chunk number: {}", chunk_index);
        nb_byte_read += read_n_bytes(&mut buf_reader, &mut chunk, base_chunk_size as u64)?;

        // If the number of bytes read is inferior to the chunk_size, it means the current BufReader has no longer data left
        if nb_byte_read < base_chunk_size {
            buf_index += 1;

            // pop a new BufReader
            buf_reader = buf_reader_vector.pop().unwrap();
        }

        // If the current chunk len is superior or equal to the size
        if chunk.len() >= base_chunk_size {
            chunk_index += 1;

            // Save the chunk into the vector before creating a new one
            chunk_vector.push(chunk);
            chunk = Vec::with_capacity(bigger_chunk_size);

            // Reset byte read counter when a chunk is full
            nb_byte_read = 0
        }
    }

    println!("File number: {}", buf_index);
    println!("Chunk number: {}", chunk_index);

    Ok(chunk_vector)
}

