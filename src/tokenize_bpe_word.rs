
// tokenize a word using calculated tokens
pub fn(&string, ordered_tokens:&OrderedSetOfTokens, unknown_token:&) 
    -> Vec<String> {
        let mut vc:Vec<String> = vec![];

        if string == "" {
            vec![]
        } 

        if ordered_tokens.len() == 0 {
            vec![unknown_token]
        }


}
