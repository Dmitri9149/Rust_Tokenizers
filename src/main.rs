use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


fn main() {
    let txt = TextStage1::build_text_stage1("alice_wonderland.txt");
//    let txt = TextStage1::replace_u2581(txt);
    let txt = TextStage1::to_lowercase(txt);
    let txt = TextStage1::separate_punctuation(txt);
    let txt = TextStage1::replace_chars_to_char(txt, "—(”)“_\\–[]\"/‘", '🦀');
    let txt = TextStage1::replace_char_to_char(txt, '🦀', ' ');
//    let txt = TextStage1::replace_chars_to_char(txt, ";:", '🦀');
//    let txt = TextStage1::separate_punctuation(txt);
    
    let txt2 = TextStage2::build_text_stage2(txt.text1);
    let voc = TextStage2::build_vocab_from_lines_ascii_ws(txt2);
    let num_tokens = TextStage2::num_tokens_s2(&voc);
    println!("{:?}", &voc.vocab);
    println!("There are {} tokens in the text", &num_tokens );

//    println!("The txt {}", &txt.text1[0..10000]);
//    println!("Hello, world!");
}


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
// 'one entity': to lowercase, to replace some symbols, to make 
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
    pub fn separate_punctuation(self) -> TextStage1 {
        let mut new_str = String::new();
//        let no_space = |(char, prev_char)| { "!;.,?:".contains(char) && prev_char != ' ' };
        
        let mut it = self.text1.chars().peekable();

        while let Some(current) = it.next() {
            if let Some(&next) = it.peek() {
                if current != ' ' &&  "!.,?:;".contains(next) {
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
        println!("in lowercase !!!!!!!!!!!!!!!!!!!");
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
pub struct TextStage2 {
    pub text0: String,
    pub vocab: HashMap<String, i32>
}

impl TextStage2 {
// build the HashMap similar to vocab from preprocessed whole string
// by splitting the string
// intended to take Text1.text1 string to 'text0' and set ''vocab' to new empty HashMap
    pub fn build_text_stage2(strng: String) -> TextStage2 {
        let voc = HashMap::new();
        TextStage2 {
            text0: strng,
            vocab: voc,
        }
    }
// build vocab: (token, count) as HashMap<String, i32>
// by splitting the 'whole string' on white spaces
//
    pub fn build_vocab_s2(mut self) -> TextStage2 {
        for word in self.text0.split_whitespace() { 
            let count = self.vocab.entry(word.to_string()).or_insert(0);
            *count +=1; 
        }  
        TextStage2 {vocab:self.vocab, ..self }
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
    pub fn build_vocab_from_lines_ws(self) -> TextStage2 {
        let mut voc = HashMap::new();
        for line in self.text0.lines() {
            for word in line.trim().split_whitespace() { 
                let count = voc.entry(word.to_string()).or_insert(0);
                *count +=1; 
            }  
        }
        TextStage2 {vocab:voc, ..self}
    }

// split the whole string on lines
// trim the lines (eliminate multiple white spaces from beginning and end)
// split_witespace -> split on whitespace 
// white space here is ASCII White_Space 
// https://doc.rust-lang.org/std/primitive.str.html#method.split_ascii_whitespace
    pub fn build_vocab_from_lines_ascii_ws(self) -> TextStage2 {
        let mut voc = HashMap::new();
        for line in self.text0.lines() {
            for word in line.trim().split_ascii_whitespace() { 
                let count = voc.entry(word.to_string()).or_insert(0);
                *count +=1; 
            }  
        }
        TextStage2 {vocab:voc, ..self}
    }

}
