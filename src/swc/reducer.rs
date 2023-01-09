use std::collections::HashMap;
use std::thread;

pub fn reduce(mut hash_map_vector: Vec<HashMap<Vec<u8>, u64>>) -> thread::Result<HashMap<Vec<u8>, u64>> {
    /*
    Takes the multiples hash map from the different mappers and merges them.
    Return the merged hash map
     */

    // Use the last vector as a result vector
    let mut res_hash_map = hash_map_vector.pop().unwrap();

    for mut hash_map in hash_map_vector {
        for (k, v) in hash_map.drain() {
            res_hash_map.entry(k).and_modify(|count| *count += v).or_insert(v);
        }
    }

    Ok(res_hash_map)
}