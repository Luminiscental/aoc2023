use crate::{day::Day, util::LineGrid};

fn is_mirror<I: Iterator<Item = (char, char)>>(zip: I, smudge: usize) -> bool {
    let mut eq = zip.filter(|(c1, c2)| c1 != c2);
    (smudge == 0 || eq.nth(smudge - 1).is_some()) && eq.next().is_none()
}

fn find_mirror<I: Iterator<Item = char>, F: Fn(usize) -> I>(
    span: usize,
    iter: F,
    smudge: usize,
) -> Option<usize> {
    (1..span).find(|&i| {
        is_mirror(
            (0..i.min(span - i)).flat_map(|j| iter(i + j).zip(iter(i - j - 1))),
            smudge,
        )
    })
}

fn score_mirror(grid: &LineGrid<'_>, smudge: usize) -> Option<usize> {
    find_mirror(grid.height(), |r| grid.iter_row(r), smudge)
        .map(|r| 100 * r)
        .or_else(|| find_mirror(grid.width(), |c| grid.iter_col(c), smudge))
}

pub struct Day13;

impl<'a> Day<'a> for Day13 {
    const DAY: usize = 13;

    type Input = Vec<LineGrid<'a>>;
    type ProcessedInput = Vec<LineGrid<'a>>;

    fn parse(input: &'a str) -> Self::Input {
        input.trim().split("\n\n").map(LineGrid::new).collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .map(|g| score_mirror(g, 0).unwrap())
            .sum::<usize>();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .iter()
            .map(|g| score_mirror(g, 1).unwrap())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day13 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_day13_examples() {
        let input = Day13::parse(EXAMPLE);
        let (input, part1) = Day13::solve_part1(input);
        let part2 = Day13::solve_part2(input);
        assert_eq!(part1, "405");
        assert_eq!(part2, "400");
    }
}
