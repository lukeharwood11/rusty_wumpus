use std::{time::Duration, thread::sleep};


// declare game.rs as a module
// we can only access public packages in game
mod game;
fn main() {

    use std::io::{ Write, stdout };
    // use the game module for experience with working with modules in other files
    game::display::print_welcome();
    let mut myString = String::new();
    for _ in 1..1000 {
        println!("{}", myString);
        sleep(Duration::from_millis(200));
        game::input::clear_terminal();
        myString.push('.');
    }

}