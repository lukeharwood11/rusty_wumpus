
pub trait Usable {
    fn use_ability(&self);
    fn to_string(&self);
}

/* Rusty Wumpus */
/*
 * The Wumpus will attack anyone who tries to go into it's home.
 * HOWEVER, the Rusty Wumpus has a lot of rusty treasure...
 * You can try to steal the treasure, but if you get caught, he'll turn you into rust.
 * 
 * After you become rust, you must solve a challenge to save yourself. If you cannot win, you lose. :)
 */
pub struct RustyWumpus {
    pub hitpoints: i32,
    pub strength: i32
}

impl Usable for RustyWumpus {
    fn to_string(&self) {
        
    }

    fn use_ability(&self) {
        
    }
}

// We are telling the compiler that the lifetime of 
// 'item's is tied to the lifetime of the Cell.
// This essentially means that we won't have a cell if we don't have
// 'a, and the compiler will hold us to that.
pub struct Cell<'a> {
    items: Vec<&'a dyn Usable>
}

pub struct Board<'a> {
    // static lifetime
    cells: [&'a Cell<'a>; 100],
}