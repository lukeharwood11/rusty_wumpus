
pub trait Item {
    pub fn use_ability(&self);
    pub fn to_string(&self);
}

pub RustyWumpus {
    
}

impl Item {
    
    fn use_ability(&self) {
        match self {
            _ =>{};
        }
    }

    fn to_string(&self) {
        match self {
            _ => {};
        }
    }
}

pub struct Cell {
    items: Vec<Item>
}

pub struct Board {
    // static lifetime
    cells: [Cell; 100],
}