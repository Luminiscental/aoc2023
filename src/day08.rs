use std::collections::HashMap;

use num::integer;

use crate::day::Day;

pub struct Input {
    instr: Vec<usize>,
    map: HashMap<[u8; 3], [[u8; 3]; 2]>,
}

pub struct Day08;

impl<'a> Day<'a> for Day08 {
    const DAY: usize = 8;

    type Input = Input;
    type ProcessedInput = Input;

    fn parse(input: &'a str) -> Self::Input {
        let (instr, map) = input.split_once("\n\n").unwrap();
        let instr = instr.trim().chars().map(|c| (c == 'R') as usize).collect();
        let map = map
            .trim()
            .lines()
            .map(|line| {
                let (p, ls) = line.split_once(" = ").unwrap();
                let (l, r) = ls.split_once(", ").unwrap();
                let encode = |s: &str| [s.as_bytes()[0], s.as_bytes()[1], s.as_bytes()[2]];
                (encode(p), [encode(&l[1..]), encode(&r[..r.len() - 1])])
            })
            .collect();
        Input { instr, map }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut node = [b'A'; 3];
        let mut steps = 0;
        for (i, &lr) in input.instr.iter().cycle().enumerate() {
            if node == [b'Z'; 3] {
                steps = i;
                break;
            }
            node = input.map.get(&node).unwrap()[lr];
        }
        (input, steps.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let cycles = input.map.keys().filter(|n| n[2] == b'A').map(|&n| {
            let mut node = n;
            let mut period = 0;
            let mut first = None;
            for (i, &lr) in input.instr.iter().cycle().enumerate() {
                if node[2] == b'Z' {
                    if let Some(j) = first {
                        assert_eq!(i, 2 * j);
                        period = j;
                        break;
                    } else {
                        first = Some(i);
                    }
                }
                node = input.map.get(&node).unwrap()[lr];
            }
            period
        });
        cycles.into_iter().fold(1, integer::lcm).to_string()
    }
}

#[cfg(test)]
mod test_day08 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const EXAMPLE2: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_day08_examples() {
        let input = Day08::parse(EXAMPLE1);
        let (_, part1) = Day08::solve_part1(input);
        assert_eq!(part1, "6");

        let input = Day08::parse(EXAMPLE2);
        let part2 = Day08::solve_part2(input);
        assert_eq!(part2, "6");
    }
}
