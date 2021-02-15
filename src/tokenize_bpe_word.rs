//use std::collections::HashMap;
use regex::Regex;
// use std::ops::Deref;
//use fancy_regex::Regex;
//use crate::vocab_of_tokens::OrderedSetOfTokens;

// tokenize a word using calculated tokens
pub fn tokenize_word(string:&str, ordered_tokens:&[String]
                     , unknown_token:&String) 
    -> Vec<String> {
        let len_sorted = ordered_tokens.len();

        if string == "" {
            return Vec::new()
        } 

        if len_sorted == 0 {
            println!("Print an unknown token !!!!!!!!!!!!!!!! {:?}", unknown_token.clone());
            return vec![unknown_token.clone()]
        }

        let mut token="****************";
        let mut token_escape;
        let mut re_token;
        let mut matched_positions;
        let mut substring_end_positions;
        let mut string_tokens = Vec::new();
        let mut interm_res;
        let mut substring_start_position;
        let mut substring;
        let remaining_substring;

        for i in 0..len_sorted {
//            println!("Length of sorted tokens in use {}", len_sorted);
//            println!("Token in use {:?}",token);
            token = &ordered_tokens[i];
            token_escape = regex::escape(&token);
            re_token = Regex::new(&token_escape).unwrap();

            matched_positions = re_token.find_iter(string);
            substring_end_positions = Vec::new();
            for mat in matched_positions {
                substring_end_positions.push(mat.start());
            }
            if substring_end_positions.len() == 0 {
                continue
            }
            
//            let mut string_tokens = Vec::new();
//            let mut interm_res =Vec::new();
            substring_start_position = 0;
            for substring_end_position in substring_end_positions {
                substring = &string[substring_start_position .. substring_end_position];
                interm_res = tokenize_word(substring, &ordered_tokens[i+1 ..], unknown_token);
                println!("Interm result {:?}", interm_res);
                string_tokens.append(&mut interm_res);
                string_tokens.push(token.to_string());
                substring_start_position = substring_end_position + token.len();
            }
            
            remaining_substring = &string[substring_start_position..];
            interm_res = tokenize_word(remaining_substring, &ordered_tokens[i+1..], unknown_token);
            string_tokens.append(&mut interm_res);
            break      
        }

        string_tokens
}

// if we have a vector of pairs , like (begin_position, end_position)
// it flattens it in vector of intermitted ...begin_position, end_position...
pub fn flatten(data: &Vec<(i32, i32)>) -> Vec<i32> {
    data
    .iter()
    .fold(Vec::with_capacity(data.len() * 2),
          |mut acc, p| { acc.push(p.0); acc.push(p.1); acc })
}

pub fn take_every_second(data:&Vec<i32>) -> Vec<&i32> {
    let mut v = Vec::with_capacity(data.len()/2);
    for item in data.iter().step_by(2) {
    v.push(item)}
    v
}

pub fn take_only_odd(data:&Vec<i32>) -> Vec<&i32> {
    let mut v = Vec::with_capacity(data.len()/2);
    for item in data.iter().skip(1).step_by(2) {
    v.push(item)} 
    v
}






