use crate::day::Day;

fn extrapolate(ns: &[i32]) -> (i32, i32) {
    let mut ends = Vec::new();
    let mut scratch = ns.to_vec();
    while scratch.iter().any(|&n| n != 0) {
        let first = scratch[0];
        for i in 0..scratch.len() - 1 {
            scratch[i] = scratch[i + 1] - scratch[i];
        }
        ends.push((first, scratch.pop().unwrap()));
    }
    ends.into_iter()
        .rev()
        .fold((0, 0), |acc, e| (e.0 - acc.0, acc.1 + e.1))
}

pub struct Day09;

impl<'a> Day<'a> for Day09 {
    const DAY: usize = 9;

    type Input = Vec<Vec<i32>>;
    type ProcessedInput = Vec<(i32, i32)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ends = input.iter().map(|ns| extrapolate(ns)).collect::<Vec<_>>();
        let ans = ends.iter().map(|e| e.1).sum::<i32>();
        (ends, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input.iter().map(|e| e.0).sum::<i32>().to_string()
    }
}

#[cfg(test)]
mod test_day09 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_day09_examples() {
        let input = Day09::parse(EXAMPLE);
        let (input, part1) = Day09::solve_part1(input);
        let part2 = Day09::solve_part2(input);
        assert_eq!(part1, "114");
        assert_eq!(part2, "2");
    }
}
