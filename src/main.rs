#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_panic)]

#[macro_use] extern crate impl_ops;

mod bitboard;
mod position;
mod file;
mod rank;

fn main() {
    let board = bitboard::BitBoard::new(0xF0FF);
    println!("{}", board);

    for p in board.into_iter() {
        print!("{} ", p);
    }
}
