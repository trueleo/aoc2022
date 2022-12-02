use crate::Solver;

struct Day01;

impl Solver for Day01 {
    type Input = String;
    type Answer = usize;

    fn solve(input: &Self::Input) -> Self::Answer {
        let input: Vec<&str> = input.split("\n").collect();

        let elves: Vec<usize> = input
            .split(|&v| v == "")
            .map(|v| v.iter().map(|&value| value.parse::<usize>().unwrap()).sum())
            .collect();

        elves.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{read_from_file, Solver};

    use super::Day01;

    #[test]
    fn five_elves_return_24000() {
        let input = read_from_file("day01_sample");
        let answer = Day01::solve(&input);
        assert_eq!(answer, 24000)
    }

    #[test]
    fn day01a_actual_input() {
        let input = read_from_file("day01");
        let answer = Day01::solve(&input);
        assert_eq!(answer, 69281)
    }
}
