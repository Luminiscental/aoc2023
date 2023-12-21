use std::collections::{HashSet, VecDeque};

use crate::{day::Day, util::LineGrid};

fn frontier(grid: &LineGrid<'_>, limit: usize, tile: bool) -> usize {
    let start = grid
        .iter()
        .find_map(|(r, c, ch)| (ch == 'S').then_some((r as i32, c as i32)))
        .unwrap();
    let mut seen = [HashSet::new(), HashSet::new()];
    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    seen[0].insert(start);
    while let Some((steps, pos)) = queue.pop_front() {
        if steps >= limit {
            break;
        }
        for dp in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let p = (pos.0 + dp.0, pos.1 + dp.1);
            let eff = if tile {
                let len = grid.width() as i32;
                (p.0.rem_euclid(len), p.1.rem_euclid(len))
            } else {
                p
            };
            if !seen[(steps + 1) % 2].contains(&p)
                && matches!(grid.try_get(eff.0, eff.1), Some(c) if c != '#')
            {
                seen[(steps + 1) % 2].insert(p);
                queue.push_back((steps + 1, p));
            }
        }
    }
    seen[limit % 2].len()
}

pub struct Day21Generic<const N1: usize, const N2: usize>;
pub type Day21 = Day21Generic<64, 26501365>;

impl<'a, const N1: usize, const N2: usize> Day<'a> for Day21Generic<N1, N2> {
    const DAY: usize = 21;

    type Input = LineGrid<'a>;
    type ProcessedInput = LineGrid<'a>;

    fn parse(input: &'a str) -> Self::Input {
        LineGrid::new(input)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = frontier(&input, N1, false);
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let s = N2 % input.width();
        let f = |n| frontier(&input, s + n * input.width(), true);
        let mut x = vec![f(0), f(1), f(2)];
        let mut dx = vec![x[1] - x[0], x[2] - x[1]];
        let mut ddx = vec![dx[1] - dx[0]];
        for i in 0.. {
            x.push(f(i + 3));
            dx.push(x[i + 3] - x[i + 2]);
            ddx.push(dx[i + 2] - dx[i + 1]);
            if ddx[i + 1] == ddx[i] {
                let j = ((N2 - s) / input.width()) as i64;
                let (i, xi, dxi, ddxi) = (i as i64, x[i] as i64, dx[i] as i64, ddx[i] as i64);
                let ext = xi + (j - i) * dxi + (j - i) * (j - i - 1) * ddxi / 2;
                return ext.to_string();
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test_day21 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_day21_examples() {
        let input = Day21Generic::<6, 5000>::parse(EXAMPLE);
        let (input, part1) = Day21Generic::<6, 5000>::solve_part1(input);
        let part2 = Day21Generic::<6, 5000>::solve_part2(input);
        assert_eq!(part1, "16");
        assert_eq!(part2, "16733044");
    }
}
