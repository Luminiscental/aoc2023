use std::{
    collections::{HashMap, HashSet},
    iter,
};

use crate::{
    util::{self, LineGrid},
    Day,
};

pub struct Day03;

impl<'a> Day<'a> for Day03 {
    const DAY: usize = 3;

    type Input = LineGrid<'a>;
    type ProcessedInput = HashMap<(usize, usize, char), Vec<u32>>;

    fn parse(input: &'a str) -> Self::Input {
        LineGrid::new(input)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut symbols = HashMap::new();
        for (i, j, c) in input.iter() {
            if !c.is_ascii_digit() && c != '.' {
                util::grid_neighbours((i, j)).for_each(|(ni, nj)| {
                    symbols.insert((ni, nj), (i, j, c));
                })
            }
        }
        let mut numbers = HashMap::new();
        for i in 0..input.height() {
            let mut start = None;
            for (j, c) in input.iter_row(i).chain(iter::once((input.width(), '.'))) {
                if c.is_ascii_digit() {
                    start = start.or(Some(j));
                } else if let Some(k) = start {
                    let number = input.section(i, k..j).parse().unwrap();
                    let mut seen = HashSet::new();
                    for l in k..j {
                        if let Some(s) = symbols.get(&(i as i32, l as i32)).copied() {
                            seen.insert(s);
                        }
                    }
                    seen.into_iter()
                        .for_each(|s| numbers.entry(s).or_insert_with(Vec::new).push(number));
                    start = None;
                }
            }
        }
        let ans = numbers.values().flat_map(|ns| ns.iter()).sum::<u32>();
        (numbers, ans.to_string())
    }

    fn solve_part2(numbers: Self::ProcessedInput) -> String {
        numbers
            .into_iter()
            .filter(|((_i, _j, c), ns)| *c == '*' && ns.len() == 2)
            .map(|(_k, ns)| ns[0] * ns[1])
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day03 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_day03_examples() {
        let input = Day03::parse(EXAMPLE);
        let (input, part1) = Day03::solve_part1(input);
        let part2 = Day03::solve_part2(input);
        assert_eq!(part1, "4361");
        assert_eq!(part2, "467835");
    }
}
