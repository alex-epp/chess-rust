use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub struct Rank(u8);


impl Rank {
    pub const fn new(r: u8) -> Self {
        assert!(r < 8);
        Self(r)
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", (self.get() + 1).to_string())
    }
}

pub const R1: Rank = Rank::new(0);
pub const R2: Rank = Rank::new(1);
pub const R3: Rank = Rank::new(2);
pub const R4: Rank = Rank::new(3);
pub const R5: Rank = Rank::new(4);
pub const R6: Rank = Rank::new(5);
pub const R7: Rank = Rank::new(6);
pub const R8: Rank = Rank::new(7);

