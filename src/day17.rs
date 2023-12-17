use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::day::Day;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
    straight: u32,
}

fn min_heat(grid: &[Vec<u32>], ultra: bool) -> Option<u32> {
    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    let get_heat = |pos: (i32, i32)| {
        (pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
            .then(|| grid[pos.0 as usize][pos.1 as usize])
    };
    let (mut heap, mut seen) = (BinaryHeap::new(), HashMap::new());
    heap.push((Reverse(0), State::default()));
    seen.insert(State::default(), 0);
    while let Some((Reverse(heat), s)) = heap.pop() {
        if seen.get(&s).map_or(false, |&h| h > heat) {
            continue;
        }
        if s.pos == (height - 1, width - 1) {
            return Some(heat);
        }
        let turns = [s.dir, (-s.dir.1, s.dir.0), (s.dir.1, -s.dir.0)];
        let starts = [(1, 0), (0, 1)];
        let dirs: &[(i32, i32)] = if s.dir == (0, 0) { &starts } else { &turns };
        for &dir in dirs {
            let straight = (s.straight + 1) * (dir == s.dir) as u32;
            let pos = (s.pos.0 + dir.0, s.pos.1 + dir.1);
            let ns = State { pos, dir, straight };
            let can_turn = !ultra || s.dir == (0, 0) || s.straight >= 3;
            let max = if ultra { 10 } else { 3 };
            let permissible = ns.straight < max && (dir == s.dir || can_turn);
            if let Some(h) = permissible.then(|| get_heat(ns.pos)).flatten() {
                if seen.get(&ns).map_or(true, |&old_h| old_h > heat + h) {
                    heap.push((Reverse(heat + h), ns));
                    seen.insert(ns, heat + h);
                }
            }
        }
    }
    None
}

pub struct Day17;

impl<'a> Day<'a> for Day17 {
    const DAY: usize = 17;

    type Input = Vec<Vec<u32>>;
    type ProcessedInput = Vec<Vec<u32>>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| line.bytes().map(|b| (b - b'0') as u32).collect())
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = min_heat(&input, false).unwrap();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        min_heat(&input, true).unwrap().to_string()
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
