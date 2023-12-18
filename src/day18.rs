use crate::day::Day;

fn area<I: Iterator<Item = (i64, i64)>>(deltas: I) -> i64 {
    let (mut x, mut y, mut int) = (0, 0, 0);
    for (dx, dy) in deltas {
        int += y * dx - x * dy + dx.abs() + dy.abs();
        x += dx;
        y += dy;
    }
    int / 2 + 1
}

pub struct Day18;

impl<'a> Day<'a> for Day18 {
    const DAY: usize = 18;

    type Input = Vec<(u8, i64, u8, i64)>;
    type ProcessedInput = Vec<(u8, i64, u8, i64)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (dir1, rest) = line.split_once(' ').unwrap();
                let (n1, color) = rest.split_once(' ').unwrap();
                let (n2, dir2) = color[2..color.len() - 1].split_at(5);
                let (dir1, dir2) = (dir1.as_bytes()[0], dir2.as_bytes()[0]);
                let (n1, n2) = (n1.parse().unwrap(), i64::from_str_radix(n2, 16).unwrap());
                (dir1, n1, dir2, n2)
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = area(input.iter().copied().map(|(dir, n, _, _)| match dir {
            b'U' => (0, n),
            b'D' => (0, -n),
            b'L' => (-n, 0),
            b'R' => (n, 0),
            _ => panic!("unrecognized direction"),
        }));
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let delta = |n, dir| [(n, 0), (0, -n), (-n, 0), (0, n)][(dir - b'0') as usize];
        area(input.into_iter().map(|(_, _, dir, n)| delta(n, dir))).to_string()
    }
}

#[cfg(test)]
mod test_day18 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_day18_examples() {
        let input = Day18::parse(EXAMPLE);
        let (input, part1) = Day18::solve_part1(input);
        let part2 = Day18::solve_part2(input);
        assert_eq!(part1, "62");
        assert_eq!(part2, "952408144115");
    }
}
