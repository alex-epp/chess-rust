use crate::{file, rank};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub struct Position(u8);

impl Position {
    pub fn new(p: u8) -> Self {
        assert!(p < 64);
        Position(p)
    }

    pub fn from_file_rank(f: file::File, r: rank::Rank) -> Self {
        Position::new(f.get() + (r.get() << 3))
    }

    pub fn get(&self) -> u8 { self.0 }
    pub fn rank(&self) -> rank::Rank {
        rank::Rank::new(self.0 / 8)
    }
    pub fn file(&self)-> file::File {
        file::File::new(self.0 % 8)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}", self.file(), self.rank())
    }
}