use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub struct File(u8);

impl File {
    pub const fn new(f: u8) -> Self {
        assert!(f < 8);
        Self(f)
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self.get() {
            0 => 'A', 1 => 'B', 2 => 'C', 3 => 'D',
            4 => 'E', 5 => 'F', 6 => 'G', 7 => 'H',
            _ => return Err(Error)
        })
    }
}

pub const A: File = File::new(0);
pub const B: File = File::new(1);
pub const C: File = File::new(2);
pub const D: File = File::new(3);
pub const E: File = File::new(4);
pub const F: File = File::new(5);
pub const G: File = File::new(6);
pub const H: File = File::new(7);