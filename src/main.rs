use std::io::prelude::*;
use std::fs::File;

fn main() {
    let txt = build_text_stage1("alice_wonderland.txt");
    let txt = TextStage1::replace_u2581(&txt);
    println!("The txt {}", txt.text1);
    println!("Hello, world!");
}

// read file in different modes

// Text is treated as one big string 
// Stage1, Stage2 are marked different stages in full string processing
// what preprocessing stages are -> see the impl of the structure and comments
pub struct TextStage1 {
    pub text1: String,
}

pub fn build_text_stage1(path: &str) -> TextStage1 {
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    TextStage1 {
        text1: contents
    }
}

impl TextStage1 {
// replace white space by u{2581} symbol 
    pub fn replace_u2581(&self) -> TextStage1 {
        let text = self.text1.replace(' ', "\u{2581}");
        TextStage1 { text1: text }
    }
}

pub struct TextStage2 {
    pub text2: String
}


