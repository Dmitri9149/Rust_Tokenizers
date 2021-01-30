use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;


fn main() {
    let txt = TextStage1::build_text_stage1("alice_wonderland.txt");
    let txt = TextStage1::replace_u2581(txt);
    let txt = TextStage1::to_lowercase(txt);
    let txt2 = TextStage2::build_text_stage2(txt.text0);
    let voc = TextStage2::build_vocab_s2(txt2);
    let num_tokens = TextStage2::num_tokens_s2(&voc);
    println!("{:?}", voc.vocab);
    println!("There are {} tokens in the text", num_tokens );

//    println!("The txt {}", &txt.text1[0..10000]);
//    println!("Hello, world!");
}
// read file in different modes
// Text is treated as one big string 
// Stage1, Stage2 are marked different stages in full string processing
// what preprocessing stages are -> see the impl of the structure and comments
pub struct TextStage1 {
// original unprocesses string
    pub text0: String,
// strings after some processings belonging to stage1
// the stage1 is for processing the initial string as 
// 'one entity': to lowercase, to replace some symbols, to make 
// may be unicode unification
    pub text1: String,
}

impl TextStage1 {
// build the string for processing in several different ways
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

// to lowercase all the string
    pub fn to_lowercase(self) -> TextStage1 {
        let text = self.text1.to_lowercase();
        TextStage1 { text1: text, ..self }    
    }
// replace non-breaking space with space
    pub fn replace_u202f(self) -> TextStage1 {
        let text = self.text1.replace('\u{202f}',&' '.to_string());
        TextStage1 { text1: text, ..self }
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
// take Text1.text0 string and set tex1 to new empty HashMap
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
// eliminate all white space characters
//
    pub fn remove_whitespace(self) -> TextStage2 {
        let text0 = self.text0
            .chars()
            .map(|x| -> char {
                if x.is_whitespace() {
                    ' '
                } else { x }
            })
            .collect();

        TextStage2 {text0:text0, ..self}   
    }
}
