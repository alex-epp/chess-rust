use crate::position::Position;
use crate::{file, rank};

use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(bb: u64) -> Self {
        Self(bb)
    }

    pub fn from_east_files(num_files: u8) -> Self {
        assert!(num_files <= 8);
        let mut files = BitBoard::new(0);
        for f in (8 - num_files) .. 8 {
            files |= Self::from(file::File::new(f));
        }
        files
    }
    pub fn from_west_files(num_files: u8) -> Self {
        assert!(num_files <= 8);
        let mut files = BitBoard::new(0);
        for f in 0 .. num_files {
            files |= Self::from(file::File::new(f));
        }
        files
    }

    pub fn is_piece_at(&self, p: Position) -> bool {
        !(self & Self::from(p)).empty()
    }
    pub fn empty(&self) -> bool {
        self.0 == 0u64
    }

    pub fn shift_n(&self) -> Self {
        self << 8
    }
    pub fn shift_n_by(&self, amount: u8) -> Self {
        self << (8 * amount)
    }
    pub fn shift_s(&self) -> Self {
        self >> 8
    }
    pub fn shift_s_by(&self, amount: u8) -> Self {
        self >> (8 * amount)
    }
    pub fn shift_e(&self) -> Self {
        (self << 1) & !Self::from(file::A)
    }
    pub fn shift_e_by(&self, amount: u8) -> Self {
        (self << amount) & !Self::from_east_files(amount)
    }
    pub fn shift_w(&self) -> Self {
        (self >> 1) & !Self::from(file::H)
    }
    pub fn shift_w_by(&self, amount: u8) -> Self {
        (self >> amount) & !Self::from_west_files(amount)
    }
    pub fn shift_ne(&self) -> Self {
        (self << 9) & !Self::from(file::A)
    }
    pub fn shift_nw(&self) -> Self {
        (self << 7) & !Self::from(file::H)
    }
    pub fn shift_se(&self) -> Self {
        (self >> 7) & !Self::from(file::A)
    }
    pub fn shift_sw(&self) -> Self {
        (self >> 9) & !Self::from(file::H)
    }

    pub fn flip_vertical(&self) -> Self {
        Self::new(self.0.swap_bytes())
    }

    pub fn fill_n_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        gen |= empty & (gen << 8);
        empty &= empty << 8;
        gen |= empty & (gen << 16);
        empty &= empty << 16;
        gen |= empty & (gen << 32);
        gen
    }
    pub fn fill_s_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        gen |= empty & (gen >> 8);
        empty &= empty >> 8;
        gen |= empty & (gen >> 16);
        empty &= empty << 16;
        gen |= empty & (gen >> 32);
        gen
    }
    pub fn fill_e_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        empty &= !BitBoard::from(file::A);
        gen |= empty & (gen << 1);
        empty &= empty << 1;
        gen |= empty & (gen << 2);
        empty &= empty << 2;
        gen |= empty & (gen << 4);
        gen
    }
    pub fn fill_w_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        empty &= !BitBoard::from(file::H);
        gen |= empty & (gen >> 1);
        empty &= empty >> 1;
        gen |= empty & (gen >> 2);
        empty &= empty << 2;
        gen |= empty & (gen >> 4);
        gen
    }
    pub fn fill_ne_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        empty &= !BitBoard::from(file::A);
        gen |= empty & (gen << 9);
        empty &= empty << 9;
        gen |= empty & (gen << 18);
        empty &= empty << 18;
        gen |= empty & (gen << 36);
        gen
    }
    pub fn fill_se_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        empty &= !BitBoard::from(file::A);
        gen |= empty & (gen >> 7);
        empty &= empty >> 7;
        gen |= empty & (gen >> 14);
        empty &= empty << 14;
        gen |= empty & (gen >> 28);
        gen
    }
    pub fn fill_nw_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        empty &= !BitBoard::from(file::H);
        gen |= empty & (gen << 7);
        empty &= empty << 7;
        gen |= empty & (gen << 14);
        empty &= empty << 14;
        gen |= empty & (gen << 28);
        gen
    }
    pub fn fill_sw_occluded(self, mut empty: Self) -> Self {
        let mut gen = self;
        empty &= !BitBoard::from(file::H);
        gen |= empty & (gen >> 9);
        empty &= empty >> 9;
        gen |= empty & (gen >> 18);
        empty &= empty << 18;
        gen |= empty & (gen >> 36);
        gen
    }

    pub fn attack_n_occluded(&self, empty: Self) -> Self {
        self.fill_n_occluded(empty).shift_n()
    }
    pub fn attack_s_occluded(&self, empty: Self) -> Self {
        self.fill_s_occluded(empty).shift_s()
    }
    pub fn attack_e_occluded(&self, empty: Self) -> Self {
        self.fill_e_occluded(empty).shift_e()
    }
    pub fn attack_w_occluded(&self, empty: Self) -> Self {
        self.fill_w_occluded(empty).shift_w()
    }
    pub fn attack_ne_occluded(&self, empty: Self) -> Self {
        self.fill_ne_occluded(empty).shift_ne()
    }
    pub fn attack_nw_occluded(&self, empty: Self) -> Self {
        self.fill_nw_occluded(empty).shift_nw()
    }
    pub fn attack_se_occluded(&self, empty: Self) -> Self {
        self.fill_se_occluded(empty).shift_se()
    }
    pub fn attack_sw_occluded(&self, empty: Self) -> Self {
        self.fill_sw_occluded(empty).shift_sw()
    }
}

impl_op_ex!(& |lhs: &BitBoard, rhs: &BitBoard| -> BitBoard { BitBoard::new(lhs.0 & rhs.0) });
impl_op_ex!(| |lhs: &BitBoard, rhs: &BitBoard| -> BitBoard { BitBoard::new(lhs.0 | rhs.0) });
impl_op_ex!(^ |lhs: &BitBoard, rhs: &BitBoard| -> BitBoard { BitBoard::new(lhs.0 ^ rhs.0) });
impl_op_ex!(<< |lhs: &BitBoard, rhs: u8| -> BitBoard { BitBoard::new(lhs.0 << rhs) });
impl_op_ex!(>> |lhs: &BitBoard, rhs: u8| -> BitBoard { BitBoard::new(lhs.0 >> rhs) });
impl_op_ex!(&= |lhs: &mut BitBoard, rhs: &BitBoard| { lhs.0 &= rhs.0 });
impl_op_ex!(|= |lhs: &mut BitBoard, rhs: &BitBoard| { lhs.0 |= rhs.0 });
impl_op_ex!(^= |lhs: &mut BitBoard, rhs: &BitBoard| { lhs.0 ^= rhs.0 });
impl_op_ex!(<<= |lhs: &mut BitBoard, rhs: u8| { lhs.0 <<= rhs });
impl_op_ex!(>>= |lhs: &mut BitBoard, rhs: u8| { lhs.0 >>= rhs });
impl_op_ex!(! |bb: &BitBoard| -> BitBoard { BitBoard::new(!bb.0) });

impl From<Position> for BitBoard {
    fn from(position: Position) -> Self {
        BitBoard::new(1u64 << position.get() as u64)
    }
}
impl From<rank::Rank> for BitBoard {
    fn from(rank: rank::Rank) -> Self {
        BitBoard::new(0xFFu64 << (8 * rank.get()) as u64)
    }
}
impl From<file::File> for BitBoard {
    fn from(file: file::File) -> Self {
        BitBoard::new(0x0101010101010101u64 << file.get() as u64)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for r in (0..8).rev() {
            for f in 0..8 {
                let pos = Position::from_file_rank(file::File::new(f), rank::Rank::new(r));
                write!(formatter, "{} ", match self.is_piece_at(pos) {
                    true => '1',
                    false => '.'
                })?;
            }
            write!(formatter, "\n")?;
        }

        Ok(())
    }
}

pub struct BitBoardIterator {
    bb: u64,
    position: u8
}
impl BitBoardIterator {
    fn new(bb: &BitBoard) -> Self {
        Self { bb: bb.0, position: 0 }
    }
}
impl Iterator for BitBoardIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.bb.trailing_zeros() as u8 + 1;
        if p != 65 {
            self.bb >>= p as u64;
            self.position += p;
            Some(Position::new(self.position - 1))
        }
        else {
            None
        }
    }
}
impl IntoIterator for &BitBoard {
    type Item = Position;
    type IntoIter = BitBoardIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}