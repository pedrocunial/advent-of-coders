use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (?<quantity>\d+) from (?<from>\d+) to (?<to>\d+)")
            .map_err(|_| InstructionParseError)?;

        let obj = re.captures(s).and_then(|cap| {
            let quantity = cap
                .name("quantity")
                .map(|q| q.as_str().parse().unwrap())
                .unwrap();

            let from = cap
                .name("from")
                .map(|q| q.as_str().parse().unwrap())
                .unwrap();

            let to = cap.name("to").map(|q| q.as_str().parse().unwrap()).unwrap();

            Some(Self {
                quantity: quantity,
                from: from,
                to: to,
            })
        });

        match obj {
            Some(instruction) => Ok(instruction),
            None => Err(InstructionParseError),
        }
    }
}
