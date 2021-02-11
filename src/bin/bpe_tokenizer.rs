use bpe::pairs::Pairs;
use bpe::TextStage1;
use bpe::VocabStage;
use bpe::vector_of_words::WordsVector;
use bpe::vocab_of_tokens::VocabOfTokens;

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
    let vec = WordsVector::string_toend(vec,"\x20\x20</word>");
    
/*    println!("==========================");
    println!("Vocab of pairs from words vector");
    println!("{:?}",&vec.words[0..20]);
    let prs = Pairs::from_words_vector(&vec);
    println!("{:?}",&prs.pairs);

    println!("==========================");
    let vocab = VocabStage::build_text_stage2("TODO! FROM STRUCT".to_string());
    let vocab = VocabStage::build_vocab_from_vector_bpe(vocab,vec);
    println!("Vocab from words vector!!!!");
    println!("{:?}", &vocab.vocab_bpe);

    println!("=========================");
    println!("Pairs from spaced words vocab");
    let prs = Pairs::from_vocab(&vocab);
    println!("{:?}", &prs.pairs);

    println!("=========================");
    println!("Get initial tokens from bpe vocab");
    println!("The initial tokens are just the unicode scalars : chars");
    let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    println!("{:?}",&tokens.tokens);
    let tokens_init_size = tokens.tokens.keys().len();
    println!("Number of initial tokens {}", tokens_init_size);
    println!("=========================");
    println!("After one merging of most frequent pairs: ");
    let vocab = VocabStage::rebuild_by_merging_pairs(vocab, ("t","h"));
    let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    println!("{:?}", &tokens.tokens);
    let tokens_aftermerge_size = tokens.tokens.keys().len();
    println!("Number of tokens after one merge {}", tokens_aftermerge_size);
*/


    let num_merges = 10;
    let vocab = VocabStage::build_text_stage2("TODO! FROM STRUCT".to_string());
    let vocab = VocabStage::build_vocab_from_vector_bpe(vocab,vec);
    let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    let prs = Pairs::from_vocab(&vocab);
    for merge in 0..num_merges {
        vocab.rebuild_by_merging_pairs(("t","h"));
        tokens.from_words_vocab_bpe_self(&vocab);
        prs.from_vocab_self(&vocab);
        let max_pair = Pairs::key_max(&prs);
        println!("Pairs is loop");
//        println!("{:?}", &prs.pairs);
//        let vocab = VocabStage::rebuild_by_merging_pairs(vocab, ("t","h"));
//        let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
//        println!("{:?}", &tokens.tokens);
    }
    println!("=========================");
    println!("After several merging of most frequent pairs: ");
//    let vocab = VocabStage::rebuild_by_merging_pairs(vocab, ("t","h"));
//    let tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
//    println!("{:?}", &tokens.tokens);
//    let tokens_aftermerge_size = tokens.tokens.keys().len();
//    println!("Number of tokens after one merge {}", tokens_aftermerge_size);




/*    let txt2 = VocabStage::build_text_stage2(txt.text1);
    let voc = VocabStage::build_vocab_from_lines_ascii_ws(txt2);
    let num_tokens = VocabStage::num_tokens_s2(&voc);
    println!("{:?}", &voc.vocab);
    println!("There are {} tokens in the text", &num_tokens );
*/

/*    println!("The txt {}", &txt.text1[0..10000]);  */
}


