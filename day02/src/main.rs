use std::{fs, str::FromStr};

use plays::{CrypticPlay, NormalPlay, Score};

mod plays;

fn calculate_play(play: &str) -> i32 {
    if let [raw_normal, raw_cryptic] = play.split(" ").collect::<Vec<&str>>()[0..2] {
        let opponent = NormalPlay::from_str(raw_normal).unwrap();
        let suggested = CrypticPlay::from_str(raw_cryptic).unwrap().to_normal();
        suggested.score() + suggested.against_normal(&opponent)
    } else {
        0
    }
}

fn main() {
    let contents = fs::read_to_string("data/test.txt").unwrap();

    let result = contents.split("\n").map(calculate_play).sum::<i32>();

    println!("{}", result);
}
