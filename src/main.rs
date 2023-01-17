// declare game.rs as a module
// we can only access public packages in game
mod game;
fn main() {

    // use the game module for experience with working with modules in other files
    game::display::print_welcome();
}