// the crate for the species structure

pub str Species {
    length: usize,
    content: String
}

pub fn vocab_with_n_length (n:i32, text:&str) -> HashMap<String, i32> {
    let hsh = HashMap::new();
    let mut char_positions = text
        .char_indices()
        .map(|(pos,_)|) pos)
        .collect_vec();
    for edge in char_positions {
        if edge + n >  char_positions.len() {
            continue;
        }
        key = &text[edge..edge+n].to_string();

    }
}
