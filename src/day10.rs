use std::collections::HashSet;

use crate::{day::Day, util::LineGrid};

fn step(grid: &LineGrid<'_>, pos: &mut (i32, i32), dir: (i32, i32)) -> Option<(i32, i32)> {
    pos.0 += dir.0;
    pos.1 += dir.1;
    match grid.get(pos.0, pos.1) {
        Some('S') => Some(dir),
        Some('|') if dir.1 == 0 => Some(dir),
        Some('-') if dir.0 == 0 => Some(dir),
        Some('7') if dir.1 == 1 => Some((1, 0)),
        Some('7') if dir.0 == -1 => Some((0, -1)),
        Some('L') if dir.1 == -1 => Some((-1, 0)),
        Some('L') if dir.0 == 1 => Some((0, 1)),
        Some('F') if dir.1 == -1 => Some((1, 0)),
        Some('F') if dir.0 == -1 => Some((0, 1)),
        Some('J') if dir.1 == 1 => Some((-1, 0)),
        Some('J') if dir.0 == 1 => Some((0, -1)),
        _ => {
            pos.0 -= dir.0;
            pos.1 -= dir.1;
            None
        }
    }
}

pub struct Day10;

impl<'a> Day<'a> for Day10 {
    const DAY: usize = 10;

    type Input = LineGrid<'a>;
    type ProcessedInput = (LineGrid<'a>, HashSet<(i32, i32)>, bool);

    fn parse(input: &'a str) -> Self::Input {
        LineGrid::new(input)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let start = input.iter().find(|&(_, _, c)| c == 'S').unwrap();
        let start_pos = (start.0 as i32, start.1 as i32);
        let mut pipes = HashSet::new();
        let mut pos = start_pos;
        pipes.insert(pos);
        let (start_dir, mut dir) = [(1, 0), (0, 1), (0, -1)]
            .into_iter()
            .find_map(|d| step(&input, &mut pos, d).map(|dir| (d, dir)))
            .unwrap();
        while pos != start_pos {
            pipes.insert(pos);
            let Some(d) = step(&input, &mut pos, dir) else {
                panic!("no connection")
            };
            dir = d;
        }
        let start_is_ilj = match (dir, start_dir) {
            (a, b) if a == b && a.1 == 0 => true,
            ((1, 0), (0, 1)) | ((0, -1), (-1, 0)) | ((0, 1), (-1, 0)) => true,
            _ => false,
        };
        let ans = pipes.len() / 2;
        ((input, pipes, start_is_ilj), ans.to_string())
    }

    fn solve_part2((input, pipes, start_is_ilj): Self::ProcessedInput) -> String {
        let mut count = 0;
        for r in 0..input.height() as i32 {
            let mut inside = false;
            for c in 0..input.width() as i32 {
                if pipes.contains(&(r, c)) {
                    match input.get(r, c) {
                        Some('|') | Some('L') | Some('J') => inside = !inside,
                        Some('S') if start_is_ilj => inside = !inside,
                        _ => (),
                    };
                } else if inside {
                    count += 1;
                }
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod test_day10 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    const EXAMPLE2: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_day10_examples() {
        let input = Day10::parse(EXAMPLE1);
        let (_, part1) = Day10::solve_part1(input);
        assert_eq!(part1, "8");

        let input = Day10::parse(EXAMPLE2);
        let (input, _) = Day10::solve_part1(input);
        let part2 = Day10::solve_part2(input);
        assert_eq!(part2, "10");
    }
}
