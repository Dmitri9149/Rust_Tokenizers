/* build and process vector of words Vec<&str> 
**
*/
use std::collections::HashMap;
use regex::Regex;
use std::ops::Deref;
//use fancy_regex::Regex;

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

pub fn merge_pairs<'a>(pairs:(&str,&str), vec:&'a Vec<&'a str>) -> Vec<String> {
    let mut vc = Vec::new();
// two ASCII spaces between tokens, will be used in regex to find the 2-spaces 
// separated tokens in the text
    let bigram = format!("{}{}{}{}{}","\x20", pairs.0,"\x20\x20",pairs.1,"\x20");
// will be used as a new token
    let glued_bigram = format!("{}{}{}{}","\x20",pairs.0,pairs.1,"\x20");
    println!("bigram {}", &bigram);
    let moc = vec![" rn t"];
// escape bigram, we may encounter in text special symbols, have to meet them literally
    let bigram_escape = regex::escape(bigram.as_str());
    let re = Regex::new(format!("{}", bigram_escape).as_str()).unwrap();
    for word in vec {
        println!("word =======> {}", &word);
        let wd = re.replace_all(word, glued_bigram.as_str()).to_string();
        vc.push(wd);
 //       println!("========== vc ======{:?}", &vc);
 //       println!("========== wd {:?}======", &wd);
    }
    return vc
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
