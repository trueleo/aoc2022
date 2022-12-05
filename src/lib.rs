use std::{fs::read_to_string, path::PathBuf, str::FromStr};

pub mod day01;
pub mod day04;

pub trait Solver {
    type Input;
    type Answer;

    fn solve(input: &Self::Input) -> Self::Answer;
}

pub fn read_from_file(file: &str) -> String {
    let mut path = PathBuf::from_str("inputs/").unwrap();
    path.push(file);

    read_to_string(path).unwrap()
}
