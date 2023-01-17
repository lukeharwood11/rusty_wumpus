use std::io::{self, Write};


fn get_input(prompt: &str) -> &str {
    // prompt the user
    println!("{}", prompt);
    // this is required to see the prompt
    let _ = io::stdout().flush();

    // create a buffer to fill
    let mut str: String = String::new();
    io::stdin().read_line(&mut str).expect("Invalid String");

    if let Some('\n') = str.chars().next_back() {
        str.pop();
    }

    if let Some('\r') = str.chars().next_back() {
        str.pop();
    }
    str
}

pub fn get_direction() {
    // get's the direction the user wants to go
    let input = get_input(">>>");
    
}
