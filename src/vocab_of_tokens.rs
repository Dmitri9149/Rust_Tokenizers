// vocab of tokens , by vocab we mean here the dictionary of 
// token:number_of_tokens pairs
pub struct Vocab_of_Tokens {
    pub tokens: HashMap<String, i32>,
}


impl Vocab_of_Tokens {
    pub fn from_words_vocab_bpe (smth: & WordsVocab) -> {
        let mut hsh = create_from_words_vocab_bpe(& smth);
        Vocab_of_Tokens {tokens:hsh}
        
    }
}


pub fn create_from_words_vocab_bpe(smth:& VocabStage) -> HashMap<String, i32> {
    let mut hsh = HashMap::new();
    for (word, frequency) in smth.words.to_iter() {
        for token in word.split_ascii_whitespace() {
             let count = hsh.entry(token.to_string()).or_insert(0);
             *count +=frequency;         
        }
    }
}
