pub trait Solver {
    type Input;
    type Answer;

    fn solve(input: &Self::Input) -> Self::Answer;
}
