use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let txt = build_TextStage1("alice_wonderland.txt");
    println!("The txt {}", txt);
    println!("Hello, world!");
}

/* read file in different variants */

/* Text is treated as one big string */ 
pub struct TextStage1 {
    pub text1: String,
}

pub fn build_TextStage1(path: &str) -> TextStage1 {
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    TextStage1 {
        text1: contents
    }

}

