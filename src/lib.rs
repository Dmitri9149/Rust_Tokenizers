#![feature(allocator_api)]

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub mod string_processing;
pub use crate::string_processing as str_mod;
pub mod vector_of_words;
pub use crate::vector_of_words as vec_words;
pub mod pairs;
pub use crate::pairs as pair;
pub mod vocab_of_tokens;
// pub use crate::vocab_of_tokens;
pub mod tokenize_bpe_word;
pub use crate::tokenize_bpe_word as word_bpe;
pub mod species;
//pub use crate::species;


// read file in different modes
// Text is treated as one big string at TextStage
// at the stage we make string processing
// what preprocessing stages are -> see the impl of the structure and comments
//
pub struct TextStage {
// original unprocesses string
    pub text0: String,
// strings after some processings belonging to stage1 are saved in text1 field
// the stage1 is for processing the initial string as 
// 'aone entity': to lowercase, to replace some symbols, to make 
// unicode unification etc...
    pub text1: String,
}

impl TextStage {
// building the string for processing
// originally build string is saved in text0 and text1 fields
// processing is with text1 field, text0 is kept without a 
// change -> the 'original string' is there 
//
// build by reading a file, no a buffer
    pub fn build_text_stage(path: &str) -> TextStage {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        TextStage {
            text0: contents.clone(),
            text1: contents
        }
    }
// replace white space by u{2581} symbol 
    pub fn replace_u2581(self) -> TextStage {
        let text = self.text1.replace(' ', "\u{2581}");
        TextStage { text1: text, ..self }
    }

// change a char to another char
    pub fn replace_char_to_char(self, x:char, y:char) -> TextStage {
        let xx = x;
        let yy = y;
        let text = self.text1.chars()
            .map(|x| -> char {
                if x==xx {
                    yy
                } else {x}
            })
        .collect();

        TextStage {
            text1:text, ..self
        }
    }

// change a chars from a list to another char
    pub fn replace_chars_to_char(self, aa:&str, b:char) -> TextStage {
//        let xx = x;
//        let yy = y;
        let text = self.text1.chars()
            .map(|x| -> char {
                if aa.contains(x) {
                    b
                } else {x}
            })
        .collect();

        TextStage {
            text1:text, ..self
        }
    }


// insert ' ' between punctuation marks '!,.' and a word 
//
    pub fn separate_punctuation(self, s:&str) -> TextStage {
        let mut new_str = String::new();
        
        let mut it = self.text1.chars().peekable();

        while let Some(current) = it.next() {
            if let Some(&next) = it.peek() {
                if current != ' ' &&  s.contains(next) {
                    new_str.push(current);
                    new_str.push(' ');
                }  else { new_str.push(current) }
            }
        }
        TextStage {text1: new_str, ..self}
    }

// to lowercase all the string
    pub fn to_lowercase(self) -> TextStage {
        let text = self.text1.to_lowercase();
        TextStage { text1: text, ..self }    
    }

//
// eliminate all white space characters
// is_whitespace -> returns true if this char has the White_Space property.
// White_Space is specified in the Unicode Character Database PropList.txt.
// https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    pub fn remove_whitespace(self) -> TextStage {
        let text0 = self.text0
            .chars()
            .map(|x| -> char {
                if x.is_whitespace() {
                    ' '
                } else { x }
            })
            .collect();

        TextStage {text0:text0, ..self}   
    }

// replace new line symbols by space (\t \n \r)
    pub fn replace_new_line(self) -> TextStage {
        let mut text = self.text1.replace('\t',&' '.to_string()); // '\t'
//        self.text1.replace('\t',&' '.to_string()); // '\t'
        text = text.replace('\n',&' '.to_string()); // '\n'
        text = text.replace('\r', &' '.to_string());


        TextStage {text1:text,  ..self }
    }
}

// 

// construct Vocab of word:number pairs 
// number here reflects the frequency on the word
// the words may have comples structure (be processed), like 
// white space inserted before every char in word, 
// or some special token may be added to the end of every word etc...
// vocab_bpe -> the words in the vocab are specially processed : 
// white space inserted before every char and to the a special token may be added
pub struct VocabStage {
    pub text0: String,
    pub vocab: HashMap<String, i32>,
    pub vocab_bpe: HashMap<String,i32>
}

impl VocabStage {

// implement 'empty' VacabStage
    pub fn new() -> VocabStage {
        let voc = HashMap::new();
        let voc_bpe = HashMap::new();
        let text0 = "".to_string();
        VocabStage {
            text0: text0,
            vocab: voc,
            vocab_bpe: voc_bpe,
        }

    }
//
// build the HashMap from preprocessed whole string
// by splitting the string
// intended to take TextStage.text1 string to 'text0' and set ''vocab' to new empty HashMap
    pub fn build_vocab_from_text_stage(strng: String) -> VocabStage {
        let voc = HashMap::new();
        let voc_bpe = HashMap::new();
        VocabStage {
            text0: strng,
            vocab: voc,
            vocab_bpe: voc_bpe,
        }
    }

    pub fn build_one_string_vocab (strng: &str) -> VocabStage {
        let mut hsh = HashMap::new();
        hsh.insert(strng.to_string(),1);
        VocabStage { 
            vocab: hsh, 
            text0: strng.to_string(),
            vocab_bpe: HashMap::new()
        }
    }

//build vocab from WordsVector

    pub fn build_vocab_from_vector(self, vec:vec_words::WordsVector) -> VocabStage {
        let vocab = vec_words::vocab_from_vector(vec.words);
        VocabStage {vocab:vocab, ..self }
    }

 // build vocab_bpe from WordsVector where words are specially preprocessed for 
 // bpe tokenizer implementation : space is added before every char in word, 
 // and special token may be added to the end
    pub fn build_vocab_from_vector_bpe(self, vec:vec_words::WordsVector) -> VocabStage {
        let vocab = vec_words::vocab_from_vector(vec.words);
        VocabStage {vocab_bpe:vocab, ..self }
    }

// build vocab: (token, count) as HashMap<String, i32>
// by splitting the 'whole string' on white spaces
//
    pub fn build_vocab_split_on_space(mut self) -> VocabStage {
        for word in self.text0.split_whitespace() { 
            let count = self.vocab.entry(word.to_string()).or_insert(0);
            *count +=1; 
        }  
        VocabStage {vocab:self.vocab, ..self }
    }
// calculate number of tokens in the vocab
    pub fn num_tokens_s2(&self) -> (usize, usize) {
        return (self.vocab.keys().len(), self.vocab_bpe.keys().len());
    }

// split the whole string on lines
// trim the lines (eliminate multiple white spaces from beginning and end)
// split_witespace -> split on whitespace 
// white space here is Unicode Derived Core Property White_Space 
// see https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
    pub fn build_vocab_from_lines_ws(self) -> VocabStage {
        let mut voc = HashMap::new();
        for line in self.text0.lines() {
            for word in line.trim().split_whitespace() { 
                let count = voc.entry(word.to_string()).or_insert(0);
                *count +=1; 
            }  
        }
        VocabStage {vocab:voc, ..self}
    }

// split the whole string on lines
// trim the lines (eliminate multiple white spaces from beginning and end)
// split_witespace -> split on whitespace 
// white space here is ASCII White_Space 
// https://doc.rust-lang.org/std/primitive.str.html#method.split_ascii_whitespace
    pub fn build_vocab_from_lines_ascii_ws(self) -> VocabStage {
        let mut voc = HashMap::new();
        for line in self.text0.lines() {
            for word in line.trim().split_ascii_whitespace() { 
                let count = voc.entry(word.to_string()).or_insert(0);
                *count +=1; 
            }  
        }
        VocabStage {vocab:voc, ..self}
    }
// one of the most important functions:
// when we iterate we find the most probable pair of strings at 
// every iteration ; the function takes the string pair , find all words 
// which have the consecutive tokens as in the pair, and merge the 
// tokens in one new string : if pair is ("aa","rd") and a 
// word is '  aa  rd  d  s  wer  </w>  ' 
// we will get : '  aard  d  s  wer  <\w>  '
   pub fn rebuild_by_merging_pairs(self, pair:(String,String)) -> VocabStage {
        let hsh = vector_of_words::merge_pairs_from_hash(pair, self.vocab_bpe);
        VocabStage {vocab_bpe:hsh, ..self}
    }
}

// some usefull functions 
// the function return the key with biggest value
fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

//fn main() {
//    let map: HashMap<_, _> = vec![(2, 4), (1, 3), (5, 2)].into_iter().collect();
//    dbg!(max_key(&map));
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_word_dict() {
        let pair = ("e","n");
        let one_word_vc = vec!["  e  n  d  p  o  w  e  r  e  n  d"];
        assert_eq!(vec!["  en  d  p  o  w  e  r  en  d"], vec_words::merge_pairs(pair, &one_word_vc));
    }
    #[test]
    fn mx() {
    let map: HashMap<_, _> = vec![(2, 4), (1, 3), (5, 2)].into_iter().collect();  
    assert_eq!(max_key(&map), Some(&2));
    }

    #[test]
    fn tokenize_word_bpe_1() {
        let ordered_tokens = ["dmi".to_string(),"tri".to_string()];
        let word = "dmitri";
        assert_eq!(vec!["dmi".to_string(), "tri".to_string()] 
                   ,word_bpe::tokenize_word(&word, &ordered_tokens, &"unc".to_string()));
    }
    #[test]
    fn tokenize_word_bpe_2() {
        let ordered_tokens = ["aaaa".to_string(),"tri".to_string()];
        let word = "dmitri";
        assert_eq!(vec!["_N_".to_string(), "tri".to_string()] 
                   ,word_bpe::tokenize_word(&word, &ordered_tokens, &"_N_".to_string()));
    }


}
