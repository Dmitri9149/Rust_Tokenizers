/* build and process vector of words Vec<&str> 
**
*/
use std::collections::HashMap;
use regex::Regex;

// white space here is Unicode Derived Core Property White_Space 
// see https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
// construct vector of all words from a string by splitting on ascii space
pub fn build_vector_of_words_ascii_ws(s:&str) -> Vec<&str> {
    let mut results = Vec::new();
    for line in s.lines() {
        for word in line.trim().split_ascii_whitespace() { 
            results.push(word);
        }  
    }
    results
}
// white space here is Unicode Derived Core Property White_Space 
// see https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
// construct vector of all words from a string by splitting on white space
pub fn build_vector_of_words_ws(s:&str) -> Vec<&str> {
    let mut results = Vec::new();
    for line in s.lines() {
        for word in line.trim().split_whitespace() { 
            results.push(word);
        }  
    }
    results
}

// puild vocab from vector of words
//
pub fn vocab_from_vector(vec:Vec<String>) -> HashMap<String,i32> {
    let mut vocab= HashMap::new();
    for word in vec.iter() {
        let count = vocab.entry(word.to_string()).or_insert(0);
        *count +=1;
    }

    vocab
}

pub fn merge_pairs<'a>(pairs:(&str,&str), vec:&'a Vec<&'a str>) -> (Vec<&'a str> , bool) {
//    let vc = Vec::new();
    let union = format!("{}{}", pairs.0, pairs.1);
    println!("union {}", &union);
    let moc = vec![" rn t"];
    let union_escape = regex::escape(union.as_str());
    let re = Regex::new(format!(r#"{}"#, union_escape).as_str()).unwrap();
//    let vec_union = vec![union];
    return (moc, re.is_match("en"))
}

/*
// build vocab: (token, count) as HashMap<String, i32>
// from words_vector
//
    pub fn build_vocab_from_vector(vec:&Vec<&str>) -> TextStage2 {
        let mut vocab = vocab_from_vector();
        TextStage2 {vocab:vocab, ..self }
    }
*/
