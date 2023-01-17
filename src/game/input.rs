use std::io::{self, Write, stdout};


fn get_input(prompt: &str) -> String {
    // prompt the user
    println!("{}", prompt);
    // this is required to see the prompt
    let _ = io::stdout().flush();

    // create a buffer to fill
    let mut ret: String = String::new();
    io::stdin().read_line(&mut ret).expect("Invalid String");

    String::from(ret.trim())
}

pub fn clear_terminal() {
    print!("\x1B[2J"); // clear the screen
    stdout().flush().unwrap(); // flush the buffer to output the escape code
}

pub fn get_direction() {
    // get's the direction the user wants to go
}
