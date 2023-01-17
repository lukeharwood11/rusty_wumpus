// include any modules you would like to 'use' by bringing into scope

// we are saying that display.rs exists, since it is in the 'game' dir rust will find it
// we also want to be able to access it in this file, so we make it public
pub mod display; 
pub mod input;
pub mod items;
pub mod user;
pub mod world;

/**
 * return the result of the game, and whether the user would like to play again
 * 
 */
pub fn start() -> (bool, bool) {
    (false, false)
}

/**
 * @return: bool indicating whether the user would like to play again
 */
pub fn play_again() -> bool {
    false
}

fn get_input() -> String {
    String::from("")
}