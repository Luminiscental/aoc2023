use std::collections::HashMap;

use crate::{day::Day, util::LineGrid};

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn energy(grid: &LineGrid<'_>, beam: ((i32, i32), usize)) -> usize {
    let (mut queue, mut seen) = (vec![beam], HashMap::new());
    while let Some(((r, c), dir)) = queue.pop() {
        let dirs: &mut u8 = seen.entry((r, c)).or_default();
        if *dirs & (1 << dir) != 0 {
            continue;
        }
        *dirs |= 1 << dir;
        let mut extend = |dirs: &[usize]| {
            queue.extend(
                dirs.iter()
                    .map(|&d| ((r + DIRS[d].0, c + DIRS[d].1), d))
                    .filter(|&((r, c), _)| grid.in_bounds(r, c)),
            );
        };
        match grid.try_get(r, c) {
            Some('/') => extend(&[3 - dir]),
            Some('\\') => extend(&[dir + 1 - 2 * (dir % 2)]),
            Some('-') if dir % 2 == 1 => extend(&[0, 2]),
            Some('|') if dir % 2 == 0 => extend(&[1, 3]),
            Some(_) => extend(&[dir]),
            None => (),
        }
    }
    seen.keys().count()
}

pub struct Day16;

impl<'a> Day<'a> for Day16 {
    const DAY: usize = 16;

    type Input = LineGrid<'a>;
    type ProcessedInput = LineGrid<'a>;

    fn parse(input: &'a str) -> Self::Input {
        LineGrid::new(input)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = energy(&input, ((0, 0), 0));
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let (w, h) = (input.width() as i32, input.height() as i32);
        let r0 = (0..h).map(|r| energy(&input, ((r, 0), 0)));
        let c0 = (0..w).map(|c| energy(&input, ((0, c), 1)));
        let r1 = (0..h).map(|r| energy(&input, ((r, w - 1), 2)));
        let c1 = (0..w).map(|c| energy(&input, ((h - 1, c), 3)));
        r0.chain(r1).chain(c0).chain(c1).max().unwrap().to_string()
    }
}

#[cfg(test)]
mod test_day16 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn test_day16_examples() {
        let input = Day16::parse(EXAMPLE);
        let (input, part1) = Day16::solve_part1(input);
        let part2 = Day16::solve_part2(input);
        assert_eq!(part1, "46");
        assert_eq!(part2, "51");
    }
}
