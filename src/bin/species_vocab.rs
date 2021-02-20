use bpe::species::{vocab_with_n_length};

use bpe::TextStage;
//use bpe::VocabStage;
//use bpe::vector_of_words::WordsVector;
fn main() {
    let txt = TextStage::build_text_stage("alice_wonderland.txt");
    let txt = TextStage::replace_new_line(txt);
    let txt = TextStage::replace_u2581(txt);
    let txt = TextStage::to_lowercase(txt);
    let vocab = vocab_with_n_length(3, &txt.text1);
    
    println!("The text looks like: {:?}", &txt.text1);

    println!("The vocab {:?}", vocab);
}
 

