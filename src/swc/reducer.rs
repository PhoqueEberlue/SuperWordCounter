use std::collections::HashMap;
use std::thread;

pub fn main_reducer(hash_map_vector: Vec<HashMap<Vec<u8>, u64>>) -> thread::Result<HashMap<Vec<u8>, u64>> {
    let mut res_hash_map = HashMap::new();

    for mut hash_map in hash_map_vector {
        for (k, v) in hash_map.drain() {
            res_hash_map.entry(k).and_modify(|count| *count += v).or_insert(v);
        }
    }

    Ok(res_hash_map)
}