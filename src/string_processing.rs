/* the module is for processing of a string as one whole string 
**
*/

// this function add symbol:char to the end of string
// 
pub fn add_symbol_toend(input: &str, symbol:char) -> String {
    let mut output = String::new();
    output.push_str(input);
    output.push(symbol);
    output
}

// this function add strng:string to the end of string input:string
// 
pub fn add_string_toend(input: &str, strng:&str) -> String {
    let mut output = String::new();
    output.push_str(input);
    output.push_str(strng);
    output
}

// this function insert space before every char in a string
// 
pub fn add_space_infront(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        output.push_str(" ");
        output.push(c);
    }
    output
}

// this function insert symbol:char before every char in a string
// 
pub fn add_char_infront(input: &str, symbol:char) -> String {
    let mut output = String::new();
    for c in input.chars() {
        output.push(symbol);
        output.push(c);
    }
    output
}

// this function insert string before every char in a string
// 
pub fn add_string_infront(input: &str, st:&str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        output.push_str(st);
        output.push(c);
    }
    output
}

// this function insert string before every but not the first char in a string
// 
pub fn infront_if_not_first_char(input: &str, st:&str) -> String {
    let mut output = String::new();
    let mut input = input.chars();
    output.push(input.next().unwrap());
    for c in input {
        output.push_str(st);
        output.push(c);
    }
    output
}


// this function insert string before every char in a string
// the chars are split in 3 groups : first char, last char and 
// all intermediate chars. string1 is added infront of first char
// string2 infront of intermediate chars
// string3 infront of last char
pub fn infront_of_every_char_3(input: &str, st1:&str, st2:&str, st3:&str, st4:&str) 
    -> String {
    let mut output = String::new();
    let mut inp = input.chars().peekable();
    let count = inp.count();
    let ln = input.len();
    if ln == 0 {
        println!("Input to infront of every cgr 3 is empty");
        return "".to_string();
    }
    
    inp = input.chars().peekable();
    if count == 1 {
        output.push_str(&["\x20\x20",st1].join(""));
        output.push(inp.next().unwrap());
        output.push_str(&[st4,"\x20\x20"].join(""));
        output
    } else if count == 2 {
        output.push_str(&format!("\x20\x20{}",st1));
        output.push(inp.next().unwrap());
        output.push_str(&format!("\x20\x20"));
        output.push(inp.next().unwrap());
        output.push_str(&format!("{}\x20\x20", st4));
        output
                
        } else {
            let last = inp.last();
            inp = input.chars().peekable();
            output.push_str(&format!("\x20\x20{}", st1));
            output.push(inp.next().unwrap());
            output.push('\x20');
            for c in inp {
                if inp.peek() == None {
                    output.push_str(&format!("\x20"));
                    output.push(last.unwrap());
                    output.push_str(&format!("{}\x20",st4));
                    break;
                }
                output.push_str(&format!("\x20{}",st2));
                output.push(inp.next().unwrap());
                output.push(c);
                output.push_str(&format!("{}\x20",st3));                               
            }
            output

        } 
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

