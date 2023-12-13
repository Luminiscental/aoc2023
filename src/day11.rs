use crate::{day::Day, util::LineGrid};

fn expand_gap(mut start: usize, mut end: usize, expands: &[usize], mult: u64) -> u64 {
    if start > end {
        (start, end) = (end, start);
    }
    let mut gap = (end - start) as u64;
    let mut i = 0;
    while i < expands.len() && expands[i] <= end {
        if expands[i] >= start {
            gap += mult - 1;
        }
        i += 1;
    }
    gap
}

fn count_distances(
    galaxies: &[(usize, usize)],
    empty_rows: &[usize],
    empty_cols: &[usize],
    mult: u64,
) -> u64 {
    let mut total = 0;
    for (i, (r0, c0)) in galaxies.iter().copied().enumerate() {
        for (r1, c1) in galaxies.iter().copied().take(i) {
            total += expand_gap(r0, r1, empty_rows, mult);
            total += expand_gap(c0, c1, empty_cols, mult);
        }
    }
    total
}

pub struct Day11Generic<const N: u64>;
pub type Day11 = Day11Generic<1000000>;

impl<'a, const N: u64> Day<'a> for Day11Generic<N> {
    const DAY: usize = 11;

    type Input = LineGrid<'a>;
    type ProcessedInput = (Vec<(usize, usize)>, Vec<usize>, Vec<usize>);

    fn parse(input: &'a str) -> Self::Input {
        LineGrid::new(input.trim())
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let galaxies = input
            .iter()
            .filter_map(|(r, c, ch)| (ch == '#').then_some((r, c)))
            .collect::<Vec<_>>();
        let mut empty_rows = (0..input.height())
            .filter(|&r| input.iter_row(r).all(|c| c == '.'))
            .collect::<Vec<_>>();
        let mut empty_cols = (0..input.width())
            .filter(|&c| (0..input.height()).all(|r| input.get(r, c) == Some('.')))
            .collect::<Vec<_>>();
        empty_rows.sort_unstable();
        empty_cols.sort_unstable();
        let ans = count_distances(&galaxies, &empty_rows, &empty_cols, 2);
        ((galaxies, empty_rows, empty_cols), ans.to_string())
    }

    fn solve_part2((galaxies, empty_rows, empty_cols): Self::ProcessedInput) -> String {
        count_distances(&galaxies, &empty_rows, &empty_cols, N).to_string()
    }
}

#[cfg(test)]
mod test_day11 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_day11_examples() {
        let input = Day11Generic::<100>::parse(EXAMPLE);
        let (input, part1) = Day11Generic::<100>::solve_part1(input);
        let part2 = Day11Generic::<100>::solve_part2(input);
        assert_eq!(part1, "374");
        assert_eq!(part2, "8410");
    }
}
