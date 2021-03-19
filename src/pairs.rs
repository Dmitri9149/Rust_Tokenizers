use std::collections::HashMap;
use crate::vector_of_words::WordsVector;
use crate::VocabStage;
use crate::max_key;

// Pais -> Dictionry of (pair of 'tokens') : some frequency measure (number)
pub struct Pairs {
    pub pairs:HashMap<(String, String),i32>
}

impl Pairs {

// implement empty Pairs
    pub fn new() -> Pairs {
        let pairs = HashMap::new();
        Pairs {pairs:pairs}
    }
// implement the Pairs from vector of words (same word may be in the vector multiple times)
// the words are special : space is inserted before every char in original word 
// so the words are like this : " p  a  i  r  s  </word>" , "  r  u  s  t  </word>"
    pub fn from_words_vector(ww:& WordsVector) -> Pairs {
        let mut hsh= HashMap::new();

        for word in &ww.words {
            let mut it = word.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = hsh
                        .entry((current.to_string(),next.to_string())).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:hsh}
    }

// get the Dictionary of (pairs): numbers from vocab in VocabStage
// the words (keys) in the vocab are special : space is inserted before every char in original word 
// so the words are like this : " p a i r s" , " r u s t"
// the 'simple' means we do not take into account how often a 
// 'word' is in vocab, we work with set of words
    pub fn from_vocab_simple(ww:& VocabStage) -> Pairs {
        let mut hsh= HashMap::new();
        for (key,_value) in &ww.vocab {
            let mut it = key.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = hsh
                        .entry((current.to_string(),next.to_string())).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:hsh}
    }

// get the Dictionary of (pairs): numbers from vocab in VocabStage
// the words (keys) in the vocab are special : space is inserted before every char in original word 
// so the words are like this : " p a i r s" , " r u s t"
// we take into account the frequencies of the words to calculate the 
// frequencies of the pairs
    pub fn from_vocab(ww:& VocabStage) -> Pairs {
        let mut hsh= HashMap::new();
        for (key,value) in &ww.vocab_bpe {
            let mut it = key.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = hsh
                        .entry((current.to_string(),next.to_string())).or_insert(0);
                    *count +=value; // we add 'value', not 1 as in case of from_vocab_simple
                }
            }
        }
        Pairs {pairs:hsh}
    }

// calculate the most frequent pair of consequtive tokens in words of VocabStage
    pub fn key_max(&self) -> (String, String) {
        let res = max_key(&self.pairs).expect("The vocabulary is to be not empty");
        (res.0.to_string(),res.1.to_string())
    }
}

