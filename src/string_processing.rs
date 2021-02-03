

// this function insert space before every char in a string
// 
pub fn add_symbol_toend(input: &str, symbol:char) -> String {
    let mut output = String::new();
    output.push_str(input);
    output.push(symbol);
    output
}

pub fn add_space_infront(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        output.push_str(" ");
        output.push(c)
    }
    output
}

// this function changes particular character in a string to particular string 
//
pub fn char_to_string(input: &str, x: char, y: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        if  c == x { output.push_str(y); 
        } else {output.push(c);}
    }
    output
}

