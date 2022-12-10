use crate::Solver;

pub enum Instruction {
    AddX(i64),
    Noop,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut tokens = s.split_ascii_whitespace();
        match tokens.next().expect("each line start with instruction") {
            "addx" => Self::AddX(
                tokens
                    .next()
                    .expect("addx gives a number")
                    .parse()
                    .expect("valid i32"),
            ),
            "noop" => Self::Noop,
            _ => unreachable!(),
        }
    }
}

pub struct Part1;

impl Solver for Part1 {
    type Input = Vec<Instruction>;

    type Answer = i64;

    fn solve(input: &Self::Input) -> Self::Answer {
        let mut important_cycles: Vec<(usize, i64)> = Vec::new();
        let mut cycle: usize = 1;
        let mut register: i64 = 1;

        for instruction in input {
            match instruction {
                Instruction::AddX(value) => {
                    cycle += 2;
                    register += value;

                    important_cycles.push((cycle, register));
                }
                Instruction::Noop => cycle += 1,
            }
        }

        let mut signal_strength_cycle = 20;
        let mut sum_of_signals = 0;
        let end_cycle = cycle;

        while signal_strength_cycle <= end_cycle {
            sum_of_signals +=
                signal_strength_cycle as i64 * during(&important_cycles, signal_strength_cycle);
            signal_strength_cycle += 40
        }

        sum_of_signals
    }
}

fn during(important_cycles: &[(usize, i64)], cycle: usize) -> i64 {
    // binary search for during
    let index = important_cycles.binary_search_by_key(&cycle, |&(cycle, _)| cycle);
    match index {
        // if there is an entry for during then return that
        Ok(index) => important_cycles[index].1,
        // else this index signifies index where cycle could have been .. just take index - 1 of that
        Err(index) => {
            // checked sub because index could be 0 .. ( not the same as cycle being 0.
            // the first important cycle is somewhere after 0)
            if let Some(index) = index.checked_sub(1) {
                important_cycles[index].1
            } else {
                // checked sub failed i.e entry was 0 so we just return default value for init register
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_from_file;

    use super::Instruction;

    #[test]
    fn test_sample() {
        let lines = read_from_file("day10_sample")
            .lines()
            .map(Instruction::from)
            .collect();

        dbg!(Part1::solve(&lines));
    }

    #[test]
    fn test_prod() {
        let lines = read_from_file("day10")
            .lines()
            .map(Instruction::from)
            .collect();

        dbg!(Part1::solve(&lines));
    }
}
