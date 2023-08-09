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
    let mut problem1_board = Board::from_str_vec(initial_position);
    let mut problem2_board = problem1_board.clone();

    let instructions = lines
        .map(Instruction::from_str)
        .map(|instruction| instruction.unwrap());

    for instruction in instructions {
        problem1_board.play_v1(instruction);
        problem2_board.play_v2(instruction);
    }

    dbg!(problem1_board.results());
    dbg!(problem2_board.results());
}
