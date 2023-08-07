use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum NormalPlay {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, EnumString, Clone)]
#[strum(ascii_case_insensitive)]
pub enum CrypticPlay {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq)]
pub enum MatchResult {
    Win,
    Draw,
    Lose,
}

pub trait Score {
    fn score(&self) -> i32;
}

trait Playable {
    fn win(&self) -> Self;
    fn lose(&self) -> Self;
    fn draw(&self) -> Self;
}

impl Playable for CrypticPlay {
    fn lose(&self) -> CrypticPlay {
        match self {
            CrypticPlay::X => CrypticPlay::Z,
            CrypticPlay::Y => CrypticPlay::X,
            CrypticPlay::Z => CrypticPlay::Y,
        }
    }

    fn draw(&self) -> CrypticPlay {
        self.clone()
    }

    fn win(&self) -> CrypticPlay {
        match self {
            CrypticPlay::X => CrypticPlay::Y,
            CrypticPlay::Y => CrypticPlay::Z,
            CrypticPlay::Z => CrypticPlay::X,
        }
    }
}

impl CrypticPlay {
    pub fn expected_play(&self, result: &MatchResult) -> Self {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Draw => self.draw(),
            MatchResult::Lose => self.lose(),
        }
    }
    pub fn to_match_result(&self) -> MatchResult {
        match self {
            CrypticPlay::X => MatchResult::Lose,
            CrypticPlay::Y => MatchResult::Draw,
            CrypticPlay::Z => MatchResult::Win,
        }
    }
}

impl NormalPlay {
    pub fn to_cryptic(&self) -> CrypticPlay {
        match self {
            NormalPlay::A => CrypticPlay::X,
            NormalPlay::B => CrypticPlay::Y,
            NormalPlay::C => CrypticPlay::Z,
        }
    }
}

impl Score for CrypticPlay {
    fn score(&self) -> i32 {
        match self {
            CrypticPlay::X => 1,
            CrypticPlay::Y => 2,
            CrypticPlay::Z => 3,
        }
    }
}

impl Score for MatchResult {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}
