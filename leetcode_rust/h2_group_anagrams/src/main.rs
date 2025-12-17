use std::collections::hash_map::HashMap;

fn main() {
    println!("Hello, world!");
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::<Vec<u8>, Vec<String>>::new();
    for s in strs {
        let mut key = s.as_bytes().to_vec();
        key.sort_unstable();
        map.entry(key).or_default().push(s);
    }
 
    return map
        .into_values()
        .collect();
}