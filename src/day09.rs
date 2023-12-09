use crate::day::Day;

fn extrapolate(ns: &mut [i32]) -> (i32, i32) {
    if ns.iter().copied().all(|n| n == 0) {
        (0, 0)
    } else {
        let (first, last_idx) = (ns[0], ns.len() - 1);
        (0..last_idx).for_each(|i| ns[i] = ns[i + 1] - ns[i]);
        let above = extrapolate(&mut ns[..last_idx]);
        (first - above.0, ns[last_idx] + above.1)
    }
}

pub struct Day09;

impl<'a> Day<'a> for Day09 {
    const DAY: usize = 9;

    type Input = Vec<Vec<i32>>;
    type ProcessedInput = i32;

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
        let (p2, p1) = input
            .into_iter()
            .map(|mut ns| extrapolate(&mut ns))
            .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
        (p2, p1.to_string())
    }

    fn solve_part2(p2: Self::ProcessedInput) -> String {
        p2.to_string()
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
