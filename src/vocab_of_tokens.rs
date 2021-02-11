use std::collections::HashMap;
use crate::VocabStage;
use crate::vector_of_words::merge_pairs_from_hash;

// vocab of tokens , by vocab we mean here the dictionary of 
// token:number_of_tokens pairs

pub struct VocabOfTokens {
    pub tokens: HashMap<String, i32>,
}


impl VocabOfTokens {
    pub fn from_words_vocab_bpe (smth: &VocabStage) -> VocabOfTokens {
        let hsh = create_from_words_vocab_bpe(&smth);
        VocabOfTokens {tokens:hsh}
     
    }
 

    pub fn from_words_vocab_bpe_self (self, smth: &VocabStage) -> VocabOfTokens {
        let hsh = create_from_words_vocab_bpe(&smth);
        VocabOfTokens {tokens:hsh}
        
    }

    pub fn merge_pair_if_in_word(smth: VocabStage, pair:(String,String)) -> VocabOfTokens {
        let hsh = merge_pairs_from_hash(pair, smth.vocab_bpe);
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

fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

//fn main() {
//    let map: HashMap<_, _> = vec![(2, 4), (1, 3), (5, 2)].into_iter().collect();
//    dbg!(max_key(&map));
// }
