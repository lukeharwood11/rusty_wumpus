use std::io::{self, Write, stdout};


fn get_input(prompt: &str) -> String {
    // prompt the user
    println!("{}", prompt);
    // this is required to see the prompt
    let _ = io::stdout().flush();

    // create a buffer to fill
    let mut ret: String = String::new();
    io::stdin().read_line(&mut ret).expect("Invalid String");

    if let Some('\n') = ret.chars().next_back() {
        ret.pop();
    }

    if let Some('\r') = ret.chars().next_back() {
        ret.pop();
    }
    ret
}

pub fn clear_terminal() {
    print!("\x1B[2J"); // clear the screen
    stdout().flush().unwrap(); // flush the buffer to output the escape code
}

pub fn get_direction() {
    // get's the direction the user wants to go
    let input = get_input(">>>");

}
