// include any modules you would like to 'use' by bringing into scope

// we are saying that display.rs exists, since it is in the 'game' dir rust will find it
pub mod display; 
mod input;
mod items;

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