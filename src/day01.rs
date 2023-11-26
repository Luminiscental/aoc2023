use crate::day::Day;

pub struct Day01 {}

impl<'a> Day<'a> for Day01 {
    const DAY: usize = 1;

    type Input = String;
    type ProcessedInput = String;

    fn parse(input: &'a str) -> Self::Input {
        input.to_owned()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        (input, "".to_owned())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        "".to_owned()
    }
}
