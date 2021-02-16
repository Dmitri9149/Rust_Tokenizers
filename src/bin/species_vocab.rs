use bpe::species::{vocab_with_n_length};

use bpe::TextStage;
//use bpe::VocabStage;
//use bpe::vector_of_words::WordsVector;
fn main() {
    let txt = TextStage::build_text_stage("alice_wonderland.txt");
    let txt = TextStage::replace_u2581(txt);
    
    println!("The text looks like: {:?}", txt);
}
 

