use std::collections::HashMap;
use std::thread;

fn reduce(mut hash_map_vector: Vec<HashMap<Vec<u8>, u64>>) -> thread::Result<HashMap<Vec<u8>, u64>> {
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

pub fn launch_reducer_threads(mut hash_map_vector_vector: Vec<Vec<HashMap<Vec<u8>, u64>>>, number_reducer: u16) -> thread::Result<HashMap<Vec<u8>, u64>> {

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
            reduce(reducer_hash_map_vector)
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

    Ok(hash_map_final)
}