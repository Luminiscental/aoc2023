use std::collections::HashMap;

use crate::day::Day;

const STEPS: usize = 1000000000;

enum State {
    Searching,
    Eating(usize, usize),
}

fn push_round<F: Fn(usize) -> (usize, usize)>(len: usize, idx: F, grid: &mut [Vec<u8>]) {
    let mut state = State::Searching;
    let mut cursor = 0;
    loop {
        let ch = (cursor < len).then(|| &mut grid[idx(cursor).0][idx(cursor).1]);
        state = match (state, ch) {
            (State::Eating(s, n), None | Some(b'#')) => {
                (s..cursor).for_each(|i| grid[idx(i).0][idx(i).1] = b'.');
                (cursor - n..cursor).for_each(|i| grid[idx(i).0][idx(i).1] = b'O');
                State::Searching
            }
            (_, None) => break,
            (State::Searching, Some(b'O')) => State::Eating(cursor, 1),
            (State::Searching, _) => State::Searching,
            (State::Eating(s, n), Some(b'O')) => State::Eating(s, n + 1),
            (State::Eating(s, n), Some(b'.')) => State::Eating(s, n),
            _ => panic!("unexpected character"),
        };
        cursor += 1;
    }
}

fn score(width: usize, height: usize, grid: &[Vec<u8>]) -> usize {
    (0..width)
        .map(|r| (0..height).filter(|&c| grid[r][c] == b'O').count() * (height - r))
        .sum::<usize>()
}

pub struct Day14;

impl<'a> Day<'a> for Day14 {
    const DAY: usize = 14;

    type Input = Vec<Vec<u8>>;
    type ProcessedInput = Vec<Vec<u8>>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|s| s.as_bytes().to_vec())
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let (w, h, mut grid) = (input[0].len(), input.len(), input.clone());
        (0..w).for_each(|c| push_round(h, |r| (h - 1 - r, c), &mut grid));
        (input, score(w, h, &grid).to_string())
    }

    fn solve_part2(mut grid: Self::ProcessedInput) -> String {
        let (w, h, mut seen) = (grid[0].len(), grid.len(), HashMap::new());
        seen.insert(grid.clone(), 0);
        let mut i = 0;
        let mut found_cycle = false;
        while i < STEPS {
            (0..w).for_each(|c| push_round(h, |r| (h - 1 - r, c), &mut grid));
            (0..h).for_each(|r| push_round(h, |c| (r, w - 1 - c), &mut grid));
            (0..w).for_each(|c| push_round(h, |r| (r, c), &mut grid));
            (0..h).for_each(|r| push_round(h, |c| (r, c), &mut grid));
            i += 1;
            if !found_cycle {
                if let Some(j) = seen.get(&grid) {
                    found_cycle = true;
                    i = STEPS - ((STEPS - i) % (i - j));
                } else {
                    seen.insert(grid.clone(), i);
                }
            }
        }
        score(w, h, &grid).to_string()
    }
}

#[cfg(test)]
mod test_day14 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_day14_examples() {
        let input = Day14::parse(EXAMPLE);
        let (input, part1) = Day14::solve_part1(input);
        let part2 = Day14::solve_part2(input);
        assert_eq!(part1, "136");
        assert_eq!(part2, "64");
    }
}
