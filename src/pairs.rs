use std::collections::HashMap;
use crate::WordsVector;
use crate::TextStage2;

// Pais -> Dictionry of (pair of 'tokens') : some frequency measure
pub struct Pairs<'a> {
    pub pairs:HashMap<(&'a str,&'a str),i32>
}

impl<'a> Pairs<'a> {
// implement the Pairs from vector of words (same word may be in the vector multiple times)
// the words are special : space is inserted before every char in original word 
// so the words are like this : " p a i r s" , " r u s t"
    pub fn from_words_vector(ww:&'a WordsVector) -> Pairs<'a> {
        let mut hsh= HashMap::new();

        for word in &ww.words {
            let mut it = word.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = hsh.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:hsh}
    }
    pub fn from_words_vector_f(mut self, ww:&'a WordsVector) -> Pairs<'a> {
        for word in &ww.words {
            let mut it = word.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = self.pairs.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:self.pairs}
    }
/*
    pub fn from_words_vector(self, ww:&WordsVector) -> Pairs<'a> {
        let mut hsh= HashMap<_,_>::new;
        for word in ww.words {
            let it = word.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = self.pairs.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:self.pairs}
    }
*/
// get the Dictionary of (pairs): numbers from vocab in TextStage2
// the words in the vocab are special : space is inserted before every char in original word 
// so the words are like this : " p a i r s" , " r u s t"
// the 'simple' means we do not take into account how often a 
// 'word' is in vocab, we work with set of words
    pub fn from_vocab_simple(ww:&'a TextStage2) -> Pairs<'a> {
        let mut hsh= HashMap::new();
        for (key,_value) in &ww.vocab {
            let mut it = key.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = hsh.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:hsh}
    }



}

