use std::{cmp::Reverse, collections::BinaryHeap};

use crate::day::Day;

fn min_heat(grid: &[Vec<i32>], min: i32, max: i32) -> Option<i32> {
    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    let mut heap = BinaryHeap::new();
    let mut seen = vec![vec![[None; 2]; width as usize]; height as usize];
    heap.push((Reverse(0), (0, 0, 0)));
    heap.push((Reverse(0), (0, 0, 1)));
    seen[0][0] = [Some(0), Some(0)];
    while let Some((Reverse(heat), (r, c, d))) = heap.pop() {
        if seen[r as usize][c as usize][d as usize].map_or(false, |h| h > heat) {
            continue;
        }
        if (r, c) == (height - 1, width - 1) {
            return Some(heat);
        }
        for (dr, dc) in [(d, 1 - d), (-d, d - 1)] {
            let mut nh = heat;
            for (n, r, c) in (1..max + 1)
                .map(|n| (n, r + n * dr, c + n * dc))
                .take_while(|&(_, r, c)| r >= 0 && c >= 0 && r < height && c < width)
            {
                nh += grid[r as usize][c as usize];
                if n >= min
                    && seen[r as usize][c as usize][1 - d as usize].map_or(true, |oh| oh > nh)
                {
                    heap.push((Reverse(nh), (r, c, 1 - d)));
                    seen[r as usize][c as usize][1 - d as usize] = Some(nh);
                }
            }
        }
    }
    None
}

pub struct Day17;

impl<'a> Day<'a> for Day17 {
    const DAY: usize = 17;

    type Input = Vec<Vec<i32>>;
    type ProcessedInput = Vec<Vec<i32>>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = min_heat(&input, 1, 3).unwrap();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        min_heat(&input, 4, 10).unwrap().to_string()
    }
}

#[cfg(test)]
mod test_day17 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_day17_examples() {
        let input = Day17::parse(EXAMPLE);
        let (input, part1) = Day17::solve_part1(input);
        let part2 = Day17::solve_part2(input);
        assert_eq!(part1, "102");
        assert_eq!(part2, "94");
    }
}
