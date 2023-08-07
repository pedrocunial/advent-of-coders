use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum NormalPlay {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum CrypticPlay {
    X,
    Y,
    Z,
}

pub trait Score {
    fn score(&self) -> i32;
    fn against_normal(&self, opponent: &Self) -> i32;
}

impl CrypticPlay {
    pub fn to_normal(&self) -> NormalPlay {
        match self {
            CrypticPlay::X => NormalPlay::A,
            CrypticPlay::Y => NormalPlay::B,
            CrypticPlay::Z => NormalPlay::C,
        }
    }
}

impl Score for NormalPlay {
    fn score(&self) -> i32 {
        match self {
            NormalPlay::A => 1,
            NormalPlay::B => 2,
            NormalPlay::C => 3,
        }
    }

    fn against_normal(&self, opponent: &Self) -> i32 {
        match [self, opponent] {
            [NormalPlay::A, NormalPlay::A] => 3,
            [NormalPlay::A, NormalPlay::B] => 0,
            [NormalPlay::A, NormalPlay::C] => 6,
            [NormalPlay::B, NormalPlay::A] => 6,
            [NormalPlay::B, NormalPlay::B] => 3,
            [NormalPlay::B, NormalPlay::C] => 0,
            [NormalPlay::C, NormalPlay::A] => 0,
            [NormalPlay::C, NormalPlay::B] => 6,
            [NormalPlay::C, NormalPlay::C] => 3,
        }
    }
}
