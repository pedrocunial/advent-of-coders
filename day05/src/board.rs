use std::collections::HashSet;

use crate::instruction::Instruction;

pub type Column = Vec<String>;
pub type Board = Vec<Column>;

pub trait Transposable
where
    Self: Sized,
{
    fn tranpose(&self, size: usize) -> Self;
}

impl Transposable for Board {
    fn tranpose(&self, size: usize) -> Self {
        (0..size)
            .map(|idx| {
                self.clone()
                    .into_iter()
                    .map(|col| col[idx].clone())
                    .collect::<Vec<_>>()
            })
            .map(|column| {
                let mut res = column
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();
                res.reverse();
                res
            })
            .collect()
    }
}

pub trait FromStrVec {
    fn from_str_vec(lines: Vec<&str>) -> Self;
}

impl FromStrVec for Board {
    fn from_str_vec(lines: Vec<&str>) -> Self {
        let size = lines[lines.len() - 1].split("  ").map(|s| s.trim()).count();

        lines[..lines.len() - 1]
            .into_iter()
            .map(|line| build_columns(&line, size))
            .collect::<Vec<_>>()
            .tranpose(size)
    }
}

pub trait Playable {
    fn play_v1(&mut self, instruction: Instruction);
    fn play_v2(&mut self, instruction: Instruction);
    fn results(&self) -> String;
}

impl Playable for Board {
    fn play_v1(&mut self, instruction: Instruction) {
        (0..instruction.quantity).for_each(|_| {
            if let Some(val) = self[instruction.from - 1].pop() {
                self[instruction.to - 1].push(val);
            }
        });
    }

    fn play_v2(&mut self, instruction: Instruction) {
        let mut crates = (0..instruction.quantity).fold(vec![], |mut acc, _| {
            if let Some(val) = self[instruction.from - 1].pop() {
                acc.push(val);
            }

            acc
        });
        crates.reverse();
        crates
            .into_iter()
            .for_each(|val| self[instruction.to - 1].push(val));
    }

    fn results(&self) -> String {
        self.into_iter()
            .filter(|col| !col.is_empty())
            .fold(vec![], |mut acc, column| {
                acc.push(column[column.len() - 1].clone());
                acc
            })
            .join("")
    }
}

fn build_columns(line: &str, size: usize) -> Column {
    let banned_chars = HashSet::from([' ', '[', ']']);

    let mut columns = line
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chars| {
            chars
                .into_iter()
                .filter(|c| !banned_chars.contains(c))
                .fold("".to_string(), |s, c| format!("{}{}", s, c).to_string())
        })
        .map(|s| vec![s])
        .flat_map(|x| x)
        .collect::<Vec<_>>();

    columns.resize(size, "".to_string());
    columns
}
