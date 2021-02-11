use bpe::TextStage;
use bpe::VocabStage;
use bpe::vector_of_words::WordsVector;
fn main() {
    let txt = TextStage::build_text_stage1("alice_wonderland.txt");
//    let txt = TextStage1::replace_u2581(txt);
    let txt = TextStage::to_lowercase(txt);
    let txt = TextStage::separate_punctuation(txt, ".,!?;:");
    let txt = TextStage::replace_chars_to_char(txt, "—(”)“_\\–[]\"/‘*", '🦀');
    let txt = TextStage::separate_punctuation(txt, ",.!?;:");
    let txt = TextStage::replace_char_to_char(txt, '🦀', ' ');
//    println!("{:?}",txt.text1)
/*    let vec = WordsVector::from_string_ws(txt);
    let vec = WordsVector::infront(vec);
*/
//    println!("{:?}",&vec.words);

    
/*    let txt2 = VocabStage::build_text_stage2(txt.text1);
    let voc = VocabStage::build_vocab_from_lines_ascii_ws(txt2);
    let num_tokens = VocabStage::num_tokens_s2(&voc);
    println!("{:?}", &voc.vocab);
    println!("There are {} tokens in the text", &num_tokens );
*/

/*    println!("The txt {}", &txt.text1[0..10000]);  */
}

