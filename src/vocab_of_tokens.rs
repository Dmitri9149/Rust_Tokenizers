use std::collections::HashMap;
use crate::VocabStage;
use crate::vector_of_words::merge_pairs_from_hash;

// vocab of tokens , by vocab we mean here the dictionary of 
// token:number_of_tokens pairs

pub struct VocabOfTokens {
    pub tokens: HashMap<String, i32>,
}


impl VocabOfTokens {

// take VocabStage vocab_bpe words (specially organized words where tokens are 
// separated by white spaces) and 
// split it on white spaces
    pub fn from_words_vocab_bpe (smth: &VocabStage) -> VocabOfTokens {
        let hsh = create_from_words_vocab_bpe(&smth);
        VocabOfTokens {tokens:hsh}
     
    }
 
// implement like a method
    pub fn from_words_vocab_bpe_self (self, smth: &VocabStage) -> VocabOfTokens {
        let hsh = create_from_words_vocab_bpe(&smth);
        VocabOfTokens {tokens:hsh}
        
    }

// merge white space separated tokens  in vocab_bpe words if the tokens 
// are in pair: argument of the function
    pub fn merge_pair_if_in_word(smth: VocabStage, pair:(String,String)) -> VocabOfTokens {
        let hsh = merge_pairs_from_hash(pair, smth.vocab_bpe);
        VocabOfTokens {tokens:hsh}
    }

}
// some helper funtions for tokens stage
// see 'from_words_vocab_bpe' above
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

// calculate length of token 
// we assume there was added special end_token 
// like ""</w>"" to the end of every word 
fn length_of_token (token: &str, end_token: &str) -> usize {
    let n = end_token.len();
    if &token[n-4 .. ] == end_token {
        &token[0 .. n-4].len()+1
    } else {
        token.len()
    }
}

//fn main() {
//    let map: HashMap<_, _> = vec![(2, 4), (1, 3), (5, 2)].into_iter().collect();
//    dbg!(max_key(&map));
// }
