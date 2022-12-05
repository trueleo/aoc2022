use crate::Solver;

struct Part1;

impl Solver for Part1 {
    type Input = String;

    type Answer = usize;

    fn solve(input: &Self::Input) -> Self::Answer {
        let ranges = input.lines().map(|line| line.split(',')).map(|ranges| {
            let mut ranges = ranges.map(|range_str| {
                let mut bounds = range_str
                    .split('-')
                    .map(|bound| bound.parse::<usize>().unwrap());
                bounds.next().expect("start bound exists")
                    ..=bounds.next().expect("end bound exists")
            });

            (
                ranges.next().expect("first range exists"),
                ranges.next().expect("second range exists"),
            )
        });

        let mut count = 0;
        for range in ranges {
            if range.0.contains(&range.1.start()) && range.0.contains(&range.1.end())
                || range.1.contains(&range.0.start()) && range.1.contains(&range.0.end())
            {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::read_from_file;

    #[test]
    fn given_sample_input_return_count() {
        let input = read_from_file("day04_sample");
        assert_eq!(Part1::solve(&input), 2);
    }

    #[test]
    fn given_prod_input_return_count() {
        let input = read_from_file("day04");
        assert_eq!(Part1::solve(&input), 507);
    }
}
