// the BPE tokenizer is composed here from modules
use bpe::pairs::Pairs;
use bpe::TextStage;
use bpe::VocabStage;
use bpe::vector_of_words::WordsVector;
use bpe::vocab_of_tokens::{VocabOfTokens, OrderedSetOfTokens};
use bpe::tokenize_bpe_word::tokenize_word;

// static UNC:String = "UNC".to_string;

fn main() {
// get text from the file 
// the text is one big string at the stage
    let txt = TextStage::build_text_stage("alice_wonderland.txt");
    let txt = TextStage::to_lowercase(txt);
    let txt = TextStage::separate_punctuation(txt, ".,!?;:");
    let txt = TextStage::replace_chars_to_char(txt, "—(”)“_\\–[]\"/‘*", '🦀');
    let txt = TextStage::separate_punctuation(txt, ",.!?;:");
    let txt = TextStage::replace_char_to_char(txt, '🦀', ' ');
//    println!("{:?}",txt.text1)
    let vec = WordsVector::from_string_ws(txt);
    let vec = WordsVector::string_infront(vec, "\x20\x20");
//    let vec = WordsVector::string_toend(vec,"\x20\x20</w>\x20\x20");
    let vec = WordsVector::string_toend(vec,"\x20\x20\u{2581}\x20\x20");

    
    println!("==========================");
    let mut vocab = VocabStage::build_vocab_from_text_stage("TODO! FROM STRUCT".to_string());
    vocab = VocabStage::build_vocab_from_vector_bpe(vocab,vec);

    println!("=========================");
    println!("Get initial tokens from bpe words vocab");
    println!("The initial tokens correspond to the unicode scalars : chars, except \u{2581} end of word");
    let mut tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
    println!("{:?}",&tokens.tokens);
    let mut tokens_size = tokens.tokens.keys().len();

    println!("Number of initial tokens {}", tokens_size);

    let num_merges = 10;
    let mut prs; // = Pairs::from_vocab(&vocab);
    let mut max_pair;
    for merge in 0..num_merges {
//          tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
          prs = Pairs::from_vocab(&vocab);
          max_pair = Pairs::key_max(&prs);
          println!("Iteration number: {}", &merge);
          println!("Max pair !!! {:?}", &max_pair);
          vocab = VocabStage::rebuild_by_merging_pairs(vocab, max_pair);
          tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);

    }
    println!("=========================");
    println!("After {} merging of most frequent pairs: ", num_merges);
    println!("The tokens vocab looks like this{:?}",&tokens.tokens);
    tokens_size = tokens.tokens.keys().len();
    println!("Number of final  tokens {}", tokens_size);
    let ordered_set = OrderedSetOfTokens::from_bpe_tokens(&tokens);

    println!("=========================");
    println!("OrderedSetOfTokens {:?}", &ordered_set.set_of_tokens);
    
    println!("=========================");
/*    let oho = tokenize_word("antidisestablishmentarianism\u{2581}"
                            ,&ordered_set.set_of_tokens[..],&"_N_".to_string());
    let uhtu = tokenize_word("hippopotomonstrosesquippedaliophobia\u{2581}"
                             ,&ordered_set.set_of_tokens[..], &"_N_".to_string());
*/
/*    let uhtu_1 = tokenize_word("hiPpopotomonstrosesquippedaliophobia\u{2581}"
                               ,&ordered_set.set_of_tokens[..]); */

    let uhtu_1 = tokenize_word("PPPPPPPaaaNNNNNNNNNNNNNN"
                               ,&ordered_set.set_of_tokens[..], "UNC");


/*    println!("Tokenize sample word ! {}", "'antidisestablishmentarianism\u{2581}'");
    println!("Oho !! {:?}", oho);
    println!("========================");
    println!("Tokenize sample word ! {}", "'hippopotomonstrosesquippedaliophobia\u{2581}'");
    println!("Hippo.... !! {:?}", uhtu);
*/
    println!("Tokenize sample word ! {}", "'hiPpopotomonstrosesquippedaliophobia\u{2581}'");

    println!("HiPpo.... !! {:?}", uhtu_1);

}
