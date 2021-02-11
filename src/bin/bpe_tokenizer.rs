use bpe::pairs::Pairs;
use bpe::TextStage;
use bpe::VocabStage;
use bpe::vector_of_words::WordsVector;
use bpe::vocab_of_tokens::VocabOfTokens;

fn main() {
// get text from the file 
// the text is one big string at the stage
    let txt = TextStage::build_text_stage("alice_wonderland.txt");
//    let txt = TextStagei::replace_u2581(txt);
    let txt = TextStage::to_lowercase(txt);
    let txt = TextStage::separate_punctuation(txt, ".,!?;:");
    let txt = TextStage::replace_chars_to_char(txt, "—(”)“_\\–[]\"/‘*", '🦀');
    let txt = TextStage::separate_punctuation(txt, ",.!?;:");
    let txt = TextStage::replace_char_to_char(txt, '🦀', ' ');
//    println!("{:?}",txt.text1)
    let vec = WordsVector::from_string_ws(txt);
    let vec = WordsVector::string_infront(vec, "\x20\x20");
    let vec = WordsVector::string_toend(vec,"\x20\x20</w>\x20\x20");
    
/*    println!("==========================");
    println!("Vocab of pairs from words vector");
    println!("{:?}",&vec.words[0..20]);
    let prs = Pairs::from_words_vector(&vec);
    println!("{:?}",&prs.pairs);
    */

    println!("==========================");
    let mut vocab = VocabStage::build_text_stage2("TODO! FROM STRUCT".to_string());
    vocab = VocabStage::build_vocab_from_vector_bpe(vocab,vec);
/*    println!("Vocab from words vector!!!!");
    println!("{:?}", &vocab.vocab_bpe);

    println!("=========================");
    println!("Pairs from spaced words vocab");
    let prs = Pairs::from_vocab(&vocab);
    println!("{:?}", &prs.pairs);
*/

    println!("=========================");
    println!("Get initial tokens from bpe vocab");
    println!("The initial tokens correspond to the unicode scalars : chars, except </w> end of word");
    let mut tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    println!("{:?}",&tokens.tokens);
    let mut tokens_size = tokens.tokens.keys().len();
    println!("Number of initial tokens {}", tokens_size);
/*    println!("=========================");
    println!("After one merging of most frequent pairs: ");
    let vocab = VocabStage::rebuild_by_merging_pairs(vocab, ("t","h"));
    let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    println!("{:?}", &tokens.tokens);
    let tokens_aftermerge_size = tokens.tokens.keys().len();
    println!("Number of tokens after one merge {}", tokens_aftermerge_size); 
*/

    let num_merges = 500;
//    let mut vocab = VocabStage::build_text_stage2("TODO! FROM STRUCT".to_string());
//    vocab = VocabStage::build_vocab_from_vector_bpe(vocab,vec);
//    let mut tokens; // = VocabOfTokens::from_words_vocab_bpe(&vocab);
    let mut prs; // = Pairs::from_vocab(&vocab);
    let mut max_pair;
    for merge in 0..num_merges {
          tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
          prs = Pairs::from_vocab(&vocab);
          max_pair = Pairs::key_max(&prs);
          println!("Iteration number: {}", &merge);
          println!("Max pair !!! {:?}", &max_pair);
//          println!("{:?}", &prs.pairs);
          vocab = VocabStage::rebuild_by_merging_pairs(vocab, max_pair);
//          println!("VocabRebuilded !!!!!!!======> {:?}",&vocab.vocab_bpe);
//        let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
//          println!("{:?}", &tokens.tokens);

    }
    println!("=========================");
    println!("After {} merging of most frequent pairs: ", num_merges);
//    let mut tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    println!("The tokens vocab looks like this{:?}",&tokens.tokens);
    tokens_size = tokens.tokens.keys().len();
    println!("Number of final  tokens {}", tokens_size);


}
