use bpe::pairs::Pairs;
use bpe::TextStage1;
use bpe::VocabStage;
use bpe::vector_of_words::WordsVector;
fn main() {
    let txt = TextStage1::build_text_stage1("alice_wonderland.txt");
//    let txt = TextStage1::replace_u2581(txt);
    let txt = TextStage1::to_lowercase(txt);
    let txt = TextStage1::separate_punctuation(txt, ".,!?;:");
    let txt = TextStage1::replace_chars_to_char(txt, "—(”)“_\\–[]\"/‘*", '🦀');
    let txt = TextStage1::separate_punctuation(txt, ",.!?;:");
    let txt = TextStage1::replace_char_to_char(txt, '🦀', ' ');
//    println!("{:?}",txt.text1)
    let vec = WordsVector::from_string_ws(txt);
    let vec = WordsVector::string_infront(vec, "\x20\x20");
    let vec = WordsVector::string_toend(vec," </word>");

    println!("{:?}",&vec.words[0..20]);
    let prs = Pairs::from_words_vector(&vec);
    println!("{:?}",&prs.pairs);


    
/*    let txt2 = VocabStage::build_text_stage2(txt.text1);
    let voc = VocabStage::build_vocab_from_lines_ascii_ws(txt2);
    let num_tokens = VocabStage::num_tokens_s2(&voc);
    println!("{:?}", &voc.vocab);
    println!("There are {} tokens in the text", &num_tokens );
*/

/*    println!("The txt {}", &txt.text1[0..10000]);  */
}


