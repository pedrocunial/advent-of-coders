use std::str::FromStr;

use board::{Board, FromStrVec};
use instruction::Instruction;

use crate::board::Playable;

mod board;
mod instruction;

fn main() {
    let input = std::fs::read_to_string("data/test.txt").unwrap();

    let mut lines = input.lines();
    let initial_position = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut board = Board::from_str_vec(initial_position);

    let instructions = lines
        .map(Instruction::from_str)
        .map(|instruction| instruction.unwrap());

    for instruction in instructions {
        board.play(instruction);
    }

    dbg!(board.results());
}
