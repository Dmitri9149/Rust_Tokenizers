/* build and process vector of words Vec<&str> 
**
*/
use std::collections::HashMap;
use regex::Regex;
// use std::ops::Deref;
//use fancy_regex::Regex;
use crate::string_processing as str_mod;
use crate::TextStage;


// collection of words we may get in some way from string
pub struct WordsVector {
    pub words: Vec <String>
}

impl WordsVector {
// white space here is Unicode Derived Core Property White_Space 
// see https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace 
// construct vector of all words from a string.text1 of 
// TextStage1 by splitting on ascii space
    pub fn from_string_ascii_ws(stage1:TextStage) -> WordsVector{
        let mut results = Vec::new();
        for line in stage1.text1.lines() {
            for word in line.trim().split_ascii_whitespace() { 
                results.push(String::from(word));
            }  
        }
        WordsVector {words: results}
    }
// white space here is Unicode Derived Core Property White_Space 
// see https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
// construct vector of all words from a string.text1 of 
// TextStage1 by splitting on white space
    pub fn from_string_ws(stage:TextStage) -> WordsVector {
        let mut results = Vec::new();
        for line in stage.text1
            .lines() {
            for word in line.trim().split_whitespace() { 
                results.push(String::from(word));
            }  
        }
        WordsVector {words:results}
    }

// generate one big single word corresponding to TextStage text
    pub fn word_as_text(stage:&TextStage)-> WordsVector{
        let text_string = vec![stage.text1.to_owned()];
        WordsVector {words:text_string}

    }

// add ' ' infront of every char in a word in words vector
    pub fn infront(vc:WordsVector) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_space_infront(x)).collect();
        WordsVector {words:results} 
    }

// add cr:char infront of every char in a word in words vector
    pub fn char_infront(vc:WordsVector, symbol:char) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_char_infront(x,symbol)).collect();
        WordsVector {words:results} 
    }

// add string infront of every char in a word in words vector
    pub fn string_infront(vc:WordsVector, st:&str) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_string_infront(x,st)).collect();
        WordsVector {words:results}
    }

// add symbol:char  to end  of every word in words-vector
    pub fn char_toend(vc:WordsVector, symbol:char) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_symbol_toend(x,symbol)).collect();
        WordsVector {words:results}
    }

// add token:string  to the end of every word in words-vector
    pub fn string_toend(vc:WordsVector, token:& str) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_string_toend(x,token)).collect();
        WordsVector {words:results}
    }

}

//================= end of WordsVector structure ===================

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

// build vocab from vector of words
//
pub fn vocab_from_vector(vec:Vec<String>) -> HashMap<String,i32> {
    let mut vocab= HashMap::new();
    for word in vec.iter() {
        let count = vocab.entry(word.to_string()).or_insert(0);
        *count +=1;
    }

    vocab
}
// merge tokens in words . where words are composed of tokens separated by 
// ASCII space symbols \x20\x20 
// take pair of tokens (strings) , construct bigram:
// format!("{}{}{}{}{}","\x20", pairs.0,"\x20\x20",pairs.1,"\x20")
// search every word in vocab for the bigram and 
// replace it by glued_bigram:
// format!("{}{}{}{}","\x20",pairs.0,pairs.1,"\x20")
pub fn merge_pairs<'a>(pairs:(&str,&str), vec:&'a Vec<&'a str>) -> Vec<String> {
    let mut vc = Vec::new();
// two ASCII spaces between tokens, will be used in regex to find the 2-spaces 
// separated tokens in the text
    let bigram = format!("{}{}{}{}{}","\x20", pairs.0,"\x20\x20",pairs.1,"\x20");
// will be used as a new token
    let glued_bigram = format!("{}{}{}{}","\x20",pairs.0,pairs.1,"\x20");
//    println!("bigram {}", &bigram);
// escape bigram, we may encounter in text special symbols, have to meet them literally
    let bigram_escape = regex::escape(bigram.as_str());
    let re = Regex::new(format!("{}", bigram_escape).as_str()).unwrap();
    for word in vec {
//        println!("word =======> {}", &word);
        let wd = re.replace_all(word, glued_bigram.as_str()).to_string();
        vc.push(wd);
    }
    return vc
}

// merge tokens in words . where words are composed of tokens separated by 
// ASCII space symbols, same as above but HashMap is a parameter
// take pair of tokens (strings) , construct bigram:
// format!("{}{}{}{}{}","\x20", pairs.0,"\x20\x20",pairs.1,"\x20")
// search for it in every word of vocab and 
// replace it by glued_bigram:
// format!("{}{}{}{}","\x20",pairs.0,pairs.1,"\x20")
pub fn merge_pairs_from_hash<'a>(pairs:(String,String), hsh: HashMap<String, i32>) -> HashMap<String,i32> {
    let mut vc = HashMap::new();
// two ASCII spaces between tokens, will be used in regex to find the 2-spaces 
// separated tokens in the text
    let bigram = format!("{}{}{}{}{}","\x20", pairs.0,"\x20\x20",pairs.1,"\x20");
// will be used as a new token
    let glued_bigram = format!("{}{}{}{}","\x20",pairs.0,pairs.1,"\x20");
    println!("glued_bigram {}", &glued_bigram);
// escape bigram, we may encounter in text special symbols, have to meet them literally
    let bigram_escape = regex::escape(bigram.as_str());
    let re = Regex::new(format!("{}", bigram_escape).as_str()).unwrap();
    for (word, frequency) in hsh {
//        println!("word =======> {}", &word);
        let wd = re.replace_all(&word, glued_bigram.as_str()).to_string();
        let count = vc.entry(wd.to_string()).or_insert(0);
        *count +=frequency;
    }
    return vc
}


