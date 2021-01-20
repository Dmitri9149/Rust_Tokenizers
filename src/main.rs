use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()>{
    let txt = TextStage1::new("alice_wonderland.txt");
    println!("The txt {}", txt);
    println!("Hello, world!");
}

/* read file in different variants */

/* Text is treated as one big string */ 
pub struct TextStage1 {
    pub text1: String,
}

impl TextStage1 {
    pub fn from_file(path: &str) -> TextStage1 {
        let mut f = File::open(path).unwrap();
        let mut contents = Vec::new();
        f.read_to_string(&mut contents).unwrap();

        }
    }
