use std::collections::HashMap;
use regex::Regex;
// use std::ops::Deref;
//use fancy_regex::Regex;
use crate::OrderedSetOfTokens;

// tokenize a word using calculated tokens
pub fn tokenize_word(&string, ordered_tokens:&OrderedSetOfTokens, unknown_token:&) 
    -> Vec<String> {
        let mut vc:Vec<String> = vec![];
        let len_sorted = ordered_tokens.len();

        if string == "" {
            vec![]
        } 

        if ordered_tokens.len() == 0 {
            vec![unknown_token]
        }

        string_tokens = vec![];
        for i in 0..len_sorted {
            token = ordered_tokens[i];
            token_reg = regex::escape(token);

            matched_position = ....to do ....
            if matched_positions.len() == 0 {
                continue
            }

            substring_end_positions = ... to do ..
            substring_start_position = 0

        }


}
