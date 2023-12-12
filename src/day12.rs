use std::collections::HashMap;

use crate::day::Day;

fn count_configurations(row: &[u8], groups: &[usize]) -> usize {
    if groups.is_empty() {
        return 1;
    }
    if row.len() < groups[0] {
        return 0;
    }
    let mut nexts: HashMap<usize, usize> = HashMap::new();
    for i in 0..row.len() - groups[0] + 1 {
        if (i..i + groups[0]).any(|j| row[j] == b'.')
            || (i > 0 && row[i - 1] == b'#')
            || (i + groups[0] < row.len() && row[i + groups[0]] == b'#')
        {
            continue;
        }
        let mut j = i + groups[0];
        if j < row.len() && row[j] == b'?' {
            j += 1;
        }
        while j < row.len() && row[j] == b'.' {
            j += 1;
        }
        *nexts.entry(j).or_default() += 1;
    }
    nexts
        .into_iter()
        .map(|(j, n)| n * count_configurations(&row[j..], &groups[1..]))
        .sum()
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
            .map(|(r, g)| count_configurations(r, g))
            .sum::<usize>();
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        "".to_string()
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
        assert_eq!(part2, "");
    }
}
