use std::collections::HashMap;
//use std::ops::Deref;
use crate::VocabStage;
use crate::vector_of_words::merge_pairs_from_hash;
//use std::iter::FromIterator;

// vocab of tokens , by vocab we mean here the dictionary of 
// token:number_of_tokens

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
// calculate entropy of the vocab
    pub fn vocab_entropy (&self) -> f32 {
        let mut sum:f32 = 0.0;
        let mut entropy:f32 = 0.0;
        for (_key,value) in &self.tokens {
            sum += *value as f32;
       }
        for (key,value) in &self.tokens {
            let f = *value as f32/sum;
            entropy -= f*f.log2();
        }

        entropy
    }

    pub fn to_value_ordered_vector(&self) -> Vec<(String,i32)> {
        let mut vc:Vec<(String,i32)> = self
            .tokens.iter()
            .map(|x| (x.0.to_owned(), x.1.to_owned()))
            .collect();  
        vc.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        vc
    }
}
//============================== end of VocabOfTokens ================
// set of tokens which are ordered by length

pub struct OrderedSetOfTokens {
    pub set_of_tokens: Vec<String>,
}

impl OrderedSetOfTokens {
    pub fn new() -> OrderedSetOfTokens {
        OrderedSetOfTokens { set_of_tokens: Vec::new(), }
    }

    pub fn from_bpe_tokens(vocab:&VocabOfTokens) -> OrderedSetOfTokens {
        let mut vc:Vec<String> = vocab.tokens.keys()
            .map(|s| s.to_string()).collect();
            vc.sort_by(|x,y| y.chars().count().cmp(&x.chars().count()));                
        OrderedSetOfTokens { set_of_tokens: vc }
    }
}
//================= end of OrderedSetOfTokens ======================
//
// some helper funtions for tokens stage
// see 'from_words_vocab_bpe' above
//
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

