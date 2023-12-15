use crate::day::Day;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |h, b| h.wrapping_add(b).wrapping_mul(17))
}

pub struct Day15;

impl<'a> Day<'a> for Day15 {
    const DAY: usize = 15;

    type Input = Vec<&'a str>;
    type ProcessedInput = Vec<&'a str>;

    fn parse(input: &'a str) -> Self::Input {
        input.trim().split(',').collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input.iter().map(|s| hash(s) as u32).sum::<u32>();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let mut boxes: [Vec<(&'a str, usize)>; 256] = std::array::from_fn(|_| Vec::new());
        for cmd in input {
            let i = cmd.find(|c| c == '=' || c == '-').unwrap();
            let h = hash(&cmd[..i]) as usize;
            let old = boxes[h].iter().position(|&(s, _)| s == &cmd[..i]);
            if cmd.as_bytes()[i] == b'-' {
                old.map(|j| boxes[h].remove(j));
            } else {
                let f = cmd[i + 1..].parse().unwrap();
                old.map(|j| boxes[h][j] = (&cmd[..i], f))
                    .unwrap_or_else(|| boxes[h].push((&cmd[..i], f)));
            }
        }
        boxes
            .into_iter()
            .enumerate()
            .map(|(i, bx)| {
                (i + 1)
                    * bx.into_iter()
                        .enumerate()
                        .map(move |(j, (_, f))| (j + 1) * f)
                        .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day15 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_day15_examples() {
        let input = Day15::parse(EXAMPLE);
        let (input, part1) = Day15::solve_part1(input);
        let part2 = Day15::solve_part2(input);
        assert_eq!(part1, "1320");
        assert_eq!(part2, "145");
    }
}
