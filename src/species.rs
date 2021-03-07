// the crate for the species structure
use std::collections::HashMap;

/*
pub str Species {
    length: usize,
    content: String
}
*/

/*
pub fn vocab_with_n_length (n:usize, text:&str) -> HashMap<String, i32> {
    let mut hsh = HashMap::new();
    let mut key = " ".to_string();
/*    let char_positions:Vec<usize> = text
        .char_indices()
        .map(|(pos,_)| pos)
        .collect();
*/
    for i in 0..text.len() {
        if i + n > text.len() {
            continue;
        }
        key = text[i..i+n].to_string();
        let count = hsh.entry(key).or_insert(0);
        *count +=1;
    }

    hsh
}
*/

pub fn vocab_with_n_length (n:usize, text:&str) -> HashMap<String, i32> {
    let mut hsh = HashMap::new();
    let mut key;
    let text_vector:Vec<_> = text
        .chars()
        .collect();
    let length = text_vector.len();

    for i in 0 .. length {
        if i + n > length {
            continue;
        }
        key = text_vector[i as usize .. i+n as usize].iter().collect();
        let count = hsh.entry(key).or_insert(0);
        *count +=1;
    }

    hsh
}


