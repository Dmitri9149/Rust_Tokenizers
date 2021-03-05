// the BPE tokenizer is composed here from modules
use bpe::pairs::Pairs;
use bpe::TextStage;
use bpe::VocabStage;
use bpe::vector_of_words::WordsVector;
use bpe::vocab_of_tokens::{VocabOfTokens, OrderedSetOfTokens};
use bpe::tokenize_bpe_word::tokenize_word;
use bpe::string_processing::prepare_for_tokenization_3;
use gnuplot::{Figure, Caption};
// use gnuplot::Color
use gnuplot::AxesCommon;
use gnuplot::Graph;


fn main() {
// get text from the file 
// the text is one big string at the stage/
    let txt = TextStage::build_text_stage("alice_wonderland.txt");
//    let txt = TextStage::build_text_stage("alllines.txt");
//    some text preprosessing before splitting on words
//    we use only lowercase words
    let txt = TextStage::to_lowercase(txt);
    let txt = TextStage::separate_punctuation(txt, ".,!?;:");
    let txt = TextStage::replace_chars_to_char(txt, "—(”)“_\\–[]\"/‘*-", '🦀');
    let txt = TextStage::separate_punctuation(txt, ",.!?;:");
    let txt = TextStage::replace_char_to_char(txt, '🦀', ' ');
//    println!("{:?}",txt.text1);
    let vec = WordsVector::from_string_ws(txt);
    let vec = WordsVector::infront_3(vec, "🔺","🔹","🔹","🔻");

// 🔹 🔸 ✔  ✔   📍  ▫️  🔻  🔺  ▪️    ▫️  ❗ 
    println!("==========================");
    println!("Words Vector for Vocab (first 20 words):\n{:?}", &vec.words[0..20]);
    
    println!("==========================");
    println!("");
// initialize Vocab
    let mut vocab = VocabStage::build_vocab_from_text_stage("TODO! FROM STRUCT".to_string());
// build Vocabulary of specially prepared words from WordsVector
    vocab = VocabStage::build_vocab_from_vector_bpe(vocab,vec);

// build vocabulary of tokens 
    let mut tokens = VocabOfTokens::from_words_vocab_bpe(&vocab);
// calculate entropy for distribution corresponding to vocabulary of tokens
    let mut entropy = tokens.vocab_entropy();
    let mut entropy_records:Vec<f32> = Vec::new();
    let mut iter_records:Vec<i32> = Vec::new();
    let mut number_of_tokens_records:Vec<usize>= Vec::new();
    let mut max_entropy:f32 = 0.;
    let mut best_merge:i32 = 0;

    entropy_records.push(entropy);
    iter_records.push(0);

    println!("=========================");
    println!("Get initial tokens from bpe words vocab");
    println!("The initial tokens correspond to the modified unicode scalars like: '🔺t', '🔹t🔹', 't🔻'");
    println!("");
    println!("{:?}\n",&tokens.tokens);
    println!("=========================\n");

    let mut tokens_size = tokens.tokens.keys().len();
    number_of_tokens_records.push(tokens_size);
    println!("Number of initial tokens {}\n", tokens_size);

    let mut ordered_tokens = tokens.to_value_ordered_vector();
    println!("=========================");
    println!("Vocab of Ordered Tokens: \n {:?}\n", ordered_tokens );
    println!("=========================");

    let num_merges = 5720;
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
          entropy= tokens.vocab_entropy();
          if entropy > max_entropy {
              max_entropy = entropy;             
              best_merge = merge;
          }

          entropy_records.push(entropy);
          iter_records.push(merge+1);
          tokens_size = tokens.tokens.keys().len();
          number_of_tokens_records.push(tokens_size);
          println!("Entropy = {:?}",entropy);
          println!("Number of Tokens : {:?}\n", tokens_size);

    }

    println!("=========================");
    println!("After {} mergings of most frequent pairs: \n", num_merges);
    println!("The tokens vocab looks like this:\n {:?}",&tokens.tokens);
    tokens_size = tokens.tokens.keys().len();
    println!("Number of final  tokens {}", tokens_size);

    let ordered_set = OrderedSetOfTokens::from_bpe_tokens(&tokens);

    println!("=========================");
    println!("OrderedSetOfTokens: \n {:?}", &ordered_set.set_of_tokens);
    
    println!("=========================");
    ordered_tokens = tokens.to_value_ordered_vector();
    println!("Vocab of Ordered Tokens: \n  {:?}", ordered_tokens );

//=========================================================================
//    let first_word = prepare_for_tokenization_3("antidisestablishmentarianism", "🔺","🔸","🔹","🔻");

    let oho_word = prepare_for_tokenization_3("antidisestablishmentarianism", "🔺","🔹","🔹","🔻");

    let uhtu_word = prepare_for_tokenization_3("hippopotomonstrosesquippedaliophobia", "🔺","🔹","🔹","🔻");

    let uhtu_1_word = prepare_for_tokenization_3("hiPpopotomonStrosesquippeDaliophobia", "🔺","🔹","🔹","🔻");

    let uhtu_2_word = prepare_for_tokenization_3("PPPPPPPabacNNNNNNNNNNNNNN", "🔺","🔹","🔹","🔻");



    let oho = tokenize_word(&oho_word
                            ,&ordered_set.set_of_tokens[..],"UNC");
    let uhtu = tokenize_word(&uhtu_word
                             ,&ordered_set.set_of_tokens[..], "UNC");

    let uhtu_1 = tokenize_word(&uhtu_1_word
                               ,&ordered_set.set_of_tokens[..], "❗");

    let uhtu_2 = tokenize_word(&uhtu_2_word
                               ,&ordered_set.set_of_tokens[..], "❗");
////////////////////////////////////////////

    let word_a = prepare_for_tokenization_3("forgetting", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("forgetting : {:?}", word_a_t);
//
    let word_a = prepare_for_tokenization_3("alice", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("alice : {:?}", word_a_t);
//
    let word_a = prepare_for_tokenization_3("yourself", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("yourself : {:?}", word_a_t);

//
    let word_a = prepare_for_tokenization_3("consented", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("consented : {:?}", word_a_t);
    
    let word_a = prepare_for_tokenization_3("inquisitively" , "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("inquisitively : {:?}", word_a_t);

    let word_a = prepare_for_tokenization_3("coronavirus", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("coronavirus : {:?}", word_a_t);

    let word_a = prepare_for_tokenization_3("tokenization", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("tokenization : {:?}", word_a_t);

    let word_a = prepare_for_tokenization_3("antidisestablishmentarianism", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("antidisestablishmentarianism : {:?}", word_a_t);

    let word_a = prepare_for_tokenization_3("hippopotomonstrosesquippedaliophobia", "🔺","🔹","🔹","🔻");
    let word_a_t = tokenize_word(&word_a
                               ,&ordered_set.set_of_tokens[..], "❗");
    println!("hippopotomonstrosesquippedaliophobia : {:?}", word_a_t);









    println!("========================");
    println!("Tokenize sample word ! {}", "antidisestablishmentarianism");
    println!("");
    println!("Oho !! {:?}", oho);
    println!("========================");
    println!("Tokenize sample word ! {}", "hippopotomonstrosesquippedaliophobia");
    println!("");
    println!("hippo.... !! {:?}\n", uhtu);
    println!("========================");
    println!("Tokenize sample word ! {}", "hiPpopotomonStrosesquippeDaliophobia");
    println!("");
    println!("hiPpo.... !! {:?}\n", uhtu_1);
    println!("========================");
    println!("Tokenize sample word ! {}", "PPPPPPPabacNNNNNNNNNNNNNN");
    println!("");

    println!(" The result is : {:?}\n",&uhtu_2);
    println!("The best merge is {}\n", best_merge);

//    println!("The entropy_records are {:?}", entropy_records);
//    println!("The Number of Tokens records are: {:?}", number_of_tokens_records);


    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Entropy  vs  number of merges", &[])
        .set_legend(Graph(0.6), Graph(0.98), &[], &[])
        .set_x_label("merges", &[])
        .set_y_label("entropy", &[])
        .lines(
            iter_records.iter(),
            entropy_records.iter(),
            &[Caption("")]);
    fg.show().unwrap();

}
