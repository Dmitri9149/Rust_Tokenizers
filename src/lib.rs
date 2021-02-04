use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub mod string_processing;
pub use crate::string_processing as str_mod;
pub mod vector_of_words;
pub use crate::vector_of_words as vec_words;
pub mod pairs;
pub use crate::pairs as pair;


// read file in different modes
// Text is treated as one big string at TextStage1
// Stage1, Stage2 are marked different stages in full string processing
// what preprocessing stages are -> see the impl of the structure and comments
//
pub struct TextStage1 {
// original unprocesses string
    pub text0: String,
// strings after some processings belonging to stage1 are saved in text1 field
// the stage1 is for processing the initial string as 
// 'aone entity': to lowercase, to replace some symbols, to make 
// unicode unification etc...
    pub text1: String,
}

impl TextStage1 {
// building the string for processing
// originally build string is saved in text0 and text1 fields
// processing is with text1 field, text0 is kept without a 
// change -> the 'original string' is there 
//
// build by reading a file, no a buffer
    pub fn build_text_stage1(path: &str) -> TextStage1 {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        TextStage1 {
            text0: contents.clone(),
            text1: contents
        }
    }
// replace white space by u{2581} symbol 
    pub fn replace_u2581(self) -> TextStage1 {
        let text = self.text1.replace(' ', "\u{2581}");
        TextStage1 { text1: text, ..self }
    }

// change a char to another char
    pub fn replace_char_to_char(self, x:char, y:char) -> TextStage1 {
        let xx = x;
        let yy = y;
        let text = self.text1.chars()
            .map(|x| -> char {
                if x==xx {
                    yy
                } else {x}
            })
        .collect();

        TextStage1 {
            text1:text, ..self
        }
    }

// change a chars from a list to another char
    pub fn replace_chars_to_char(self, aa:&str, b:char) -> TextStage1 {
//        let xx = x;
//        let yy = y;
        let text = self.text1.chars()
            .map(|x| -> char {
                if aa.contains(x) {
                    b
                } else {x}
            })
        .collect();

        TextStage1 {
            text1:text, ..self
        }
    }


// insert ' ' between punctuation marks '!,.' and a word 
//
    pub fn separate_punctuation(self, s:&str) -> TextStage1 {
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
        TextStage1 {text1: new_str, ..self}
    }

// to lowercase all the string
    pub fn to_lowercase(self) -> TextStage1 {
        let text = self.text1.to_lowercase();
        TextStage1 { text1: text, ..self }    
    }

//
// eliminate all white space characters
// is_whitespace -> returns true if this char has the White_Space property.
// White_Space is specified in the Unicode Character Database PropList.txt.
// https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt
    pub fn remove_whitespace(self) -> TextStage1 {
        let text0 = self.text0
            .chars()
            .map(|x| -> char {
                if x.is_whitespace() {
                    ' '
                } else { x }
            })
            .collect();

        TextStage1 {text0:text0, ..self}   
    }

// replace non-breaking spaces by space
    pub fn replace_non_breaking(self) -> TextStage1 {
        let text = self.text1.replace('\t',&' '.to_string()); // '\t'

        TextStage1 { text1: text, ..self }
    }
}

// keep String from which to build vocab
// split the string , build vocab from splitted parts
pub struct VocabStage {
    pub text0: String,
    pub vocab: HashMap<String, i32>
}

impl VocabStage {
// build the HashMap similar to vocab from preprocessed whole string
// by splitting the string
// intended to take Text1.text1 string to 'text0' and set ''vocab' to new empty HashMap
    pub fn build_text_stage2(strng: String) -> VocabStage {
        let voc = HashMap::new();
        VocabStage {
            text0: strng,
            vocab: voc,
        }
    }

//build vocab from WordsVector

    pub fn build_vocab_from_vector(self, vec:WordsVector) -> VocabStage {
        let vocab = vec_words::vocab_from_vector(vec.words);
        VocabStage {vocab:vocab, ..self }
    }




// build vocab: (token, count) as HashMap<String, i32>
// by splitting the 'whole string' on white spaces
//
    pub fn build_vocab_s2(mut self) -> VocabStage {
        for word in self.text0.split_whitespace() { 
            let count = self.vocab.entry(word.to_string()).or_insert(0);
            *count +=1; 
        }  
        Vec {vocab:self.vocab, ..self }
    }
// calculate number of tokens in the vocab
    pub fn num_tokens_s2(&self) -> usize {
        return self.vocab.keys().len();
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
}
// insert space before every char in every word in vocab
//    pub fn space_infront(self) -> TextStage2 {
//        for word in self.vocab {
//
//        }
//    }


// collection of words we may get in some way from string
pub struct WordsVector {
    pub words: Vec <String>
}

impl WordsVector {
// white space here is Unicode Derived Core Property White_Space 
// see https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace 
// construct vector of all words from a string.text1 of 
// TextStage1 by splitting on ascii space
    pub fn from_string_ascii_ws(stage1:TextStage1) -> WordsVector{
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
    pub fn from_string_ws(stage1:TextStage1) -> WordsVector {
        let mut results = Vec::new();
        for line in stage1.text1
            .lines() {
            for word in line.trim().split_whitespace() { 
                results.push(String::from(word));
            }  
        }
        WordsVector {words:results}
    }

// add ' ' infront of every char in a word in words vector
    pub fn infront(vc:WordsVector) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_space_infront(x)).collect();
        WordsVector {words:results} 
    }

// add symbol:char  to end  of every word in words-vector
    pub fn toend(vc:WordsVector, symbol:char) -> WordsVector {
        let results = vc.words.iter()
            .map(|x| str_mod::add_symbol_toend(x,symbol)).collect();
        WordsVector {words:results} 
    }
}
