/* build and process vector of words Vec<&str> 
**
*/
use std::collections::HashMap;
//use regex::Regex;
use fancy_regex::Regex;

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
    let vc = Vec::new();
    let bigram = format!("{}{}{}", pairs.0,' ',pairs.1);
    let glued_bigram = format!("{}{}",pairs.0,pairs.1);
    println!("bigram {}", &bigram);
    let moc = vec![" rn t"];
    let bigram_escape = regex::escape(bigram.as_str());
    let re = Regex::new(format!(r#"{}{}{}"#, "(?<!\\S)",bigram_escape,"(?!\\S)").as_str()).unwrap();
    for word in vec {
        let wd = re.replace_all(word, glued_bigram.as_str());
        vc.push(wd);
    }
    return (moc, re.is_match("e n").is_ok())
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
