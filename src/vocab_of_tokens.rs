use std::collections::HashMap;
use crate::VocabStage;

// vocab of tokens , by vocab we mean here the dictionary of 
// token:number_of_tokens pairs

pub struct VocabOfTokens {
    pub tokens: HashMap<String, i32>,
}


impl VocabOfTokens {
    pub fn from_words_vocab_bpe (smth: & VocabStage) -> VocabOfTokens {
        let hsh = create_from_words_vocab_bpe(& smth);
        VocabOfTokens {tokens:hsh}
        
    }

    pub fn merge_pair_if_in_word(self, pair:(&str,&str)) -> VocabOfTokens {
        let hsh = merge_pairs_from_hash(pair, self.tokens);
        VocabOfTokens {tokens:hsh}
    }

}

pub fn create_from_words_vocab_bpe(smth:& VocabStage) -> HashMap<String,i32>{
    let mut hsh = HashMap::new();
    for (word, frequency) in &smth.vocab_bpe {
        for token in word.split_ascii_whitespace() {
             let count = hsh.entry(token.to_string()).or_insert(0);
             *count +=frequency;         
        }
    }

    hsh
}
