use std::{collections::HashMap, iter};

use itertools::{intersperse, Itertools};

use crate::day::Day;

fn count(row: &[u8], groups: &[usize], memo: &mut HashMap<(Vec<u8>, Vec<usize>), usize>) -> usize {
    let k = (row.to_vec(), groups.to_vec());
    memo.get(&k).copied().unwrap_or_else(|| {
        if groups.is_empty() {
            return row.iter().copied().all(|c| c != b'#') as usize;
        }
        if row.len() < groups[0] {
            return 0;
        }
        let mut nexts: HashMap<usize, usize> = HashMap::new();
        for i in 0..row.len() - groups[0] + 1 {
            let (end, len) = (i + groups[0], row.len());
            if i > 0 && row[i - 1] == b'#' {
                break;
            }
            if (i..end).any(|j| row[j] == b'.') || (end < len && row[end] == b'#') {
                continue;
            }
            let j = end + (end < len && row[end] == b'?') as usize;
            let j = (j..len).find(|&j| row[j] != b'.').unwrap_or(len);
            *nexts.entry(j).or_default() += 1;
        }
        let result = nexts
            .into_iter()
            .map(|(j, n)| n * count(&row[j..], &groups[1..], memo))
            .sum();
        memo.insert(k, result);
        result
    })
}

pub struct Day12;

impl<'a> Day<'a> for Day12 {
    const DAY: usize = 12;

    type Input = Vec<(&'a [u8], Vec<usize>)>;
    type ProcessedInput = Vec<(&'a [u8], Vec<usize>)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (springs, groups) = line.split_once(' ').unwrap();
                let groups = groups.split(',').map(|n| n.parse().unwrap()).collect();
                (springs.as_bytes(), groups)
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .map(move |(r, g)| count(r, g, &mut HashMap::new()))
            .sum::<usize>();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let mut memo = HashMap::new();
        input
            .into_iter()
            .map(move |(r, g)| {
                let r5 = intersperse(iter::repeat(r.to_vec()).take(5), vec![b'?']);
                let g5 = iter::repeat(g).take(5);
                count(&r5.concat(), &g5.concat(), &mut memo)
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day12 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn test_day12_examples() {
        let input = Day12::parse(EXAMPLE);
        let (input, part1) = Day12::solve_part1(input);
        let part2 = Day12::solve_part2(input);
        assert_eq!(part1, "21");
        assert_eq!(part2, "525152");
    }
}
