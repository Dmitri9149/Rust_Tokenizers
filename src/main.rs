use std::io::prelude::*;
use std::fs::File;

fn main() {
    let txt = TextStage1::build_text_stage1("alice_wonderland.txt");
    let txt = TextStage1::replace_u2581(txt);
    let txt = TextStage1::to_lowercase(txt);

    println!("The txt {}", txt.text1);
    println!("Hello, world!");
}

// read file in different modes

// Text is treated as one big string 
// Stage1, Stage2 are marked different stages in full string processing
// what preprocessing stages are -> see the impl of the structure and comments
pub struct TextStage1 {
// original unprocesses string
    pub text0: String,
// strings after some processings belonging to stage1
    pub text1: String,
}

impl TextStage1 {
// build the string for processing in several different ways

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
}

pub struct TextStage2 {
    pub text2: String
}


