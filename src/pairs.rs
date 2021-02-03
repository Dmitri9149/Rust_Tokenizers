use std::collections::HashMap;
use crate::WordsVector;
use crate::TextStage2;


pub struct Pairs<'a> {
    pairs:HashMap<(&'a str,&'a str),i32>
}

impl<'a> Pairs<'a> {
// make empty Pairs
/*    pub fn empty() -> Pairs<'_,'_> {
        let e = HashMap::new;
        Pairs {pairs:e}
    }
*/
    pub fn from_words_vector(mut self, ww:&'a WordsVector) -> Pairs<'a> {
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
    pub fn from_vocab(mut self, ww:&'a TextStage2) -> Pairs<'a> {
        for (key,_value) in &ww.vocab {
            let mut it = key.split_whitespace().peekable();

            while let Some(current) = it.next() {
                if let Some(&next) = it.peek() {
                    let count = self.pairs.entry((current,next)).or_insert(0);
                    *count +=1;          
                }
            }
        }
        Pairs {pairs:self.pairs}
    }



}

